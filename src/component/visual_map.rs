use serde::Serialize;

use crate::basic::color::Color;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Continuous {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<(f64, f64)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calculable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    realtime: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    inverse: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    precision: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_width: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_height: Option<f64>,
}

impl Continuous {
    pub fn new() -> Self {
        Self {
            type_: "continuous".to_string(),
            min: None,
            max: None,
            range: None,
            calculable: None,
            realtime: None,
            inverse: None,
            precision: None,
            item_width: None,
            item_height: None,
        }
    }

    pub fn min<F: Into<f64>>(mut self, min: F) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<F: Into<f64>>(mut self, max: F) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn range<F: Into<f64>>(mut self, range: (F, F)) -> Self {
        self.range = Some((range.0.into(), range.1.into()));
        self
    }

    pub fn calculable(mut self, calculable: bool) -> Self {
        self.calculable = Some(calculable);
        self
    }

    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }

    pub fn inverse(mut self, inverse: bool) -> Self {
        self.inverse = Some(inverse);
        self
    }

    pub fn precision<F: Into<f64>>(mut self, precision: F) -> Self {
        self.precision = Some(precision.into());
        self
    }

    pub fn item_width<F: Into<f64>>(mut self, item_width: F) -> Self {
        self.item_width = Some(item_width.into());
        self
    }

    pub fn item_height<F: Into<f64>>(mut self, item_height: F) -> Self {
        self.item_height = Some(item_height.into());
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Piece {
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
}

impl Piece {
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
            label: None,
            color: None,
        }
    }

    pub fn min<F: Into<f64>>(mut self, min: F) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<F: Into<f64>>(mut self, max: F) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl From<(f64, f64)> for Piece {
    fn from((min, max): (f64, f64)) -> Self {
        Self::new().min(min).max(max)
    }
}

impl From<(i64, i64)> for Piece {
    fn from((min, max): (i64, i64)) -> Self {
        Self::new().min(min as f64).max(max as f64)
    }
}

impl From<(f64, f64, &str)> for Piece {
    fn from((min, max, label): (f64, f64, &str)) -> Self {
        Self::new().min(min).max(max).label(label)
    }
}

impl From<(i64, i64, &str)> for Piece {
    fn from((min, max, label): (i64, i64, &str)) -> Self {
        Self::new().min(min as f64).max(max as f64).label(label)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Piecewise {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    series_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pieces: Option<Vec<Piece>>,
}

impl Piecewise {
    pub fn new() -> Self {
        Self {
            type_: "piecewise".to_string(),
            show: None,
            dimension: None,
            series_index: None,
            pieces: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn dimension<F: Into<f64>>(mut self, dimension: F) -> Self {
        self.dimension = Some(dimension.into());
        self
    }

    pub fn series_index<F: Into<f64>>(mut self, series_index: F) -> Self {
        self.series_index = Some(series_index.into());
        self
    }

    pub fn pieces(mut self, pieces: Vec<Piece>) -> Self {
        self.pieces = Some(pieces);
        self
    }
}

pub enum Type {
    Continuous(Continuous),
    Piecewise(Piecewise),
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Type::Continuous(c) => c.serialize(serializer),
            Type::Piecewise(p) => p.serialize(serializer),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualMap(Vec<Type>);

impl VisualMap {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn continuous(mut self, continuous: Continuous) -> Self {
        self.0.push(Type::Continuous(continuous));
        self
    }

    pub fn piecewise(mut self, piecewise: Piecewise) -> Self {
        self.0.push(Type::Piecewise(piecewise));
        self
    }
}
