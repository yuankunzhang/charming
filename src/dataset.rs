pub struct Dataset {
    pub source: Vec<DataPoints>,
}

type DataPoints = serde_json::Value;

#[cfg(test)]
mod test {
    use serde_json::{json, Value};

    use super::DataPoints;

    #[test]
    fn series() {
        let data_points: DataPoints = json!(["Matcha Latte", 43.3, 85.8, 93.7]);
        match data_points {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 4);
                match &arr[0] {
                    Value::String(s) => assert_eq!(s, "Matcha Latte"),
                    _ => panic!("Expected first element to be a string"),
                }
                match &arr[1] {
                    Value::Number(n) => assert_eq!(n.as_f64().unwrap(), 43.3),
                    _ => panic!("Expected second element to be a number"),
                }
            }
            _ => panic!("Expected series to be an array"),
        }
    }
}
