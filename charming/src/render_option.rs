use serde::Serialize;
use serde_content::{Data, Serializer, Value};

use crate::{
    Chart, EchartsError,
    element::{js_expression::EXPRESSION_SERDE_NAME, js_function::FUNCTION_SERDE_NAME},
};

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
enum PathPart {
    Key(String),
    Index(usize),
}

#[derive(Debug, PartialEq)]
enum CodeKind {
    Function,
    Expression,
}

struct CodeSlot {
    path: Vec<PathPart>,
    source: String,
    kind: CodeKind,
}

pub(crate) struct RenderOption {
    data: Value<'static>,
    slots: Vec<CodeSlot>,
}

impl RenderOption {
    pub(crate) fn new(chart: &Chart) -> Result<Self, EchartsError> {
        let mut data = Serializer::new()
            .human_readable()
            .serialize(chart)
            .map_err(serialization_error)?;
        let mut slots = Vec::new();
        extract(&mut data, &mut Vec::new(), &mut slots)?;
        Ok(Self { data, slots })
    }

    pub(crate) fn to_javascript(&self) -> Result<String, EchartsError> {
        let json = serde_json::to_string(&self.data).map_err(serialization_error)?;
        let literal = serde_json::to_string(&json)
            .map_err(serialization_error)?
            .replace('<', "\\u003c");
        let mut script = format!("((values) => {{\nconst option = JSON.parse({literal});\n");
        for (index, slot) in self.slots.iter().enumerate() {
            script.push_str(&format!("{{\nconst value = values[{index}];\n"));
            if slot.kind == CodeKind::Function {
                script.push_str("if (typeof value !== 'function') throw new TypeError('expected a JavaScript function');\n");
            }
            script.push_str("option");
            for part in &slot.path {
                script.push('[');
                script.push_str(&serde_json::to_string(part).map_err(serialization_error)?);
                script.push(']');
            }
            script.push_str(" = value;\n}\n");
        }
        // Evaluate source outside the assembly scope so its helpers cannot shadow globals.
        script.push_str("return option;\n})([\n");
        for slot in &self.slots {
            script.push_str("(\n");
            script.push_str(&slot.source);
            script.push_str("\n),\n");
        }
        script.push_str("])");
        Ok(script)
    }

    #[cfg(feature = "wasm")]
    pub(crate) fn to_wasm(&self) -> Result<wasm_bindgen::JsValue, EchartsError> {
        use js_sys::Reflect;
        use wasm_bindgen::JsValue;

        let option = self
            .data
            .serialize(&serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true))
            .map_err(|error| EchartsError::WasmError(error.to_string()))?;
        for slot in &self.slots {
            let value = js_sys::eval(&format!("(\n{}\n)", slot.source)).map_err(wasm_error)?;
            if slot.kind == CodeKind::Function && !value.is_function() {
                return Err(EchartsError::WasmError(
                    "expected a JavaScript function".into(),
                ));
            }
            let mut target = option.clone();
            let (last, parents) = slot.path.split_last().expect("chart code has a field path");
            let key = |part: &PathPart| match part {
                PathPart::Key(key) => JsValue::from_str(key),
                PathPart::Index(index) => JsValue::from_f64(*index as f64),
            };
            for part in parents {
                target = Reflect::get(&target, &key(part)).map_err(wasm_error)?;
            }
            Reflect::set(&target, &key(last), &value).map_err(wasm_error)?;
        }
        Ok(option)
    }
}

fn serialization_error(error: impl std::fmt::Display) -> EchartsError {
    EchartsError::SerializationError(error.to_string())
}

#[cfg(feature = "wasm")]
pub(crate) fn wasm_error(error: wasm_bindgen::JsValue) -> EchartsError {
    EchartsError::WasmError(format!("{error:?}"))
}

fn extract(
    value: &mut Value<'_>,
    path: &mut Vec<PathPart>,
    slots: &mut Vec<CodeSlot>,
) -> Result<(), EchartsError> {
    if let Value::Struct(st) = value {
        let kind = match st.name.as_ref() {
            FUNCTION_SERDE_NAME => Some(CodeKind::Function),
            EXPRESSION_SERDE_NAME => Some(CodeKind::Expression),
            _ => None,
        };
        if let Some(kind) = kind {
            let Data::Struct { fields } = &st.data else {
                return Err(serialization_error("invalid JavaScript source struct"));
            };
            let source = fields
                .iter()
                .find_map(|(key, value)| match (key.as_ref(), value) {
                    ("source", Value::String(source)) => Some(source.to_string()),
                    _ => None,
                })
                .ok_or_else(|| serialization_error("missing JavaScript source"))?;
            slots.push(CodeSlot {
                path: path.clone(),
                source,
                kind,
            });
            *value = Value::Unit;
            return Ok(());
        }
    }
    match value {
        Value::Option(Some(value)) => extract(value, path, slots)?,
        Value::Struct(st) => extract_data(&mut st.data, path, slots)?,
        Value::Enum(en) => {
            if en.data != Data::Unit {
                path.push(PathPart::Key(en.variant.to_string()));
                extract_data(&mut en.data, path, slots)?;
                path.pop();
            }
        }
        Value::Seq(values) | Value::Tuple(values) => extract_values(values, path, slots)?,
        Value::Map(entries) => {
            for (key, value) in entries {
                let Value::String(key) = key else {
                    return Err(serialization_error("chart object keys must be strings"));
                };
                path.push(PathPart::Key(key.to_string()));
                extract(value, path, slots)?;
                path.pop();
            }
        }
        _ => {}
    }
    Ok(())
}

fn extract_data(
    data: &mut Data<'_>,
    path: &mut Vec<PathPart>,
    slots: &mut Vec<CodeSlot>,
) -> Result<(), EchartsError> {
    match data {
        Data::Unit => {}
        Data::NewType { value } => extract(value, path, slots)?,
        Data::Tuple { values } => extract_values(values, path, slots)?,
        Data::Struct { fields } => {
            for (key, value) in fields {
                path.push(PathPart::Key(key.to_string()));
                extract(value, path, slots)?;
                path.pop();
            }
        }
    }
    Ok(())
}

fn extract_values(
    values: &mut [Value<'_>],
    path: &mut Vec<PathPart>,
    slots: &mut Vec<CodeSlot>,
) -> Result<(), EchartsError> {
    for (index, value) in values.iter_mut().enumerate() {
        path.push(PathPart::Index(index));
        extract(value, path, slots)?;
        path.pop();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::JsFunction;
    use serde_json::json;

    #[test]
    fn typed_slots_follow_serialized_paths_and_lookalike_data_stays_data() {
        #[derive(Serialize)]
        struct Wrapper(JsFunction);
        #[derive(Serialize)]
        enum Callback {
            Named { callback: Wrapper },
            Tuple(Wrapper, i32),
            Newtype(Wrapper),
        }
        let function = || Wrapper(JsFunction::from("function() {}"));
        let plain = json!({"type":"function", "source":"throw new Error('data')"});
        let input = (
            Some(vec![
                Callback::Named {
                    callback: function(),
                },
                Callback::Tuple(function(), 1),
                Callback::Newtype(function()),
            ]),
            plain.clone(),
            "#*#*#*#literal#*#*#*#",
        );
        let mut data = Serializer::new()
            .human_readable()
            .serialize(&input)
            .unwrap();
        let mut slots = Vec::new();
        extract(&mut data, &mut Vec::new(), &mut slots).unwrap();
        let key = |s: &str| PathPart::Key(s.to_string());
        assert_eq!(
            slots.iter().map(|slot| &slot.path).collect::<Vec<_>>(),
            vec![
                &vec![
                    PathPart::Index(0),
                    PathPart::Index(0),
                    key("Named"),
                    key("callback")
                ],
                &vec![
                    PathPart::Index(0),
                    PathPart::Index(1),
                    key("Tuple"),
                    PathPart::Index(0)
                ],
                &vec![PathPart::Index(0), PathPart::Index(2), key("Newtype")],
            ]
        );
        assert_eq!(
            serde_json::to_value(data).unwrap(),
            json!([
                [{"Named":{"callback":null}}, {"Tuple":[null,1]}, {"Newtype":null}],
                plain, "#*#*#*#literal#*#*#*#",
            ])
        );
    }
}
