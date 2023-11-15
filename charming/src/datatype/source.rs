use serde::Serialize;

use super::CompositeValue;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum DataSource {
    Integers(Vec<Vec<i64>>),
    Floats(Vec<Vec<f64>>),
    Mixed(Vec<Vec<CompositeValue>>),
}

impl From<Vec<Vec<i32>>> for DataSource {
    fn from(v: Vec<Vec<i32>>) -> Self {
        let t: Vec<Vec<i64>> = v
            .iter()
            .map(|x| x.iter().map(|y| *y as i64).collect())
            .collect();
        DataSource::Integers(t)
    }
}

impl From<Vec<Vec<i64>>> for DataSource {
    fn from(v: Vec<Vec<i64>>) -> Self {
        DataSource::Integers(v)
    }
}

impl From<Vec<Vec<f32>>> for DataSource {
    fn from(v: Vec<Vec<f32>>) -> Self {
        let t: Vec<Vec<f64>> = v
            .iter()
            .map(|x| x.iter().map(|y| *y as f64).collect())
            .collect();
        DataSource::Floats(t)
    }
}

impl From<Vec<Vec<f64>>> for DataSource {
    fn from(v: Vec<Vec<f64>>) -> Self {
        DataSource::Floats(v)
    }
}

impl From<Vec<Vec<CompositeValue>>> for DataSource {
    fn from(v: Vec<Vec<CompositeValue>>) -> Self {
        DataSource::Mixed(v)
    }
}

#[macro_export]
macro_rules! ds {
    ($([$($x:expr),* $(,)?]),* $(,)?) => {
        $crate::datatype::DataSource::from(vec![
            $(
                vec![
                    $(
                        $crate::datatype::CompositeValue::from($x)
                    ),*
                ]
            ),*
        ])
    };
}

#[cfg(test)]
mod test {

    use crate::datatype::NumericValue;

    use super::*;

    #[test]
    fn numeric_value_from_i32() {
        let n: NumericValue = 42i32.into();
        assert_eq!(n, NumericValue::Integer(42));
    }

    #[test]
    fn numeric_value_from_i64() {
        let n: NumericValue = 42i64.into();
        assert_eq!(n, NumericValue::Integer(42));
    }

    #[test]
    #[should_panic]
    fn numeric_value_from_f32() {
        let n: NumericValue = 0.618f32.into();
        assert_eq!(n, NumericValue::Float(0.618));
    }

    #[test]
    fn numeric_value_from_f64() {
        let n: NumericValue = 0.618f64.into();
        assert_eq!(n, NumericValue::Float(0.618));
    }

    #[test]
    fn composite_value_from_numeric_value() {
        let n: CompositeValue = NumericValue::Integer(42).into();
        assert_eq!(n, CompositeValue::Number(NumericValue::Integer(42)));
    }

    #[test]
    fn composite_value_from_str() {
        let s: CompositeValue = "Monday".into();
        assert_eq!(s, CompositeValue::String("Monday".to_string()));
    }

    #[test]
    fn data_frame_from_integers() {
        let ds: DataSource = vec![vec![1i32, 2i32, 3i32], vec![4i32, 5i32, 6i32]].into();
        assert_eq!(
            ds,
            DataSource::Integers(vec![vec![1i64, 2i64, 3i64], vec![4i64, 5i64, 6i64]])
        );
    }

    #[test]
    fn data_frame_from_bigintegers() {
        let ds: DataSource = vec![vec![1i64, 2i64, 3i64], vec![4i64, 5i64, 6i64]].into();
        assert_eq!(
            ds,
            DataSource::Integers(vec![vec![1i64, 2i64, 3i64], vec![4i64, 5i64, 6i64]])
        );
    }

    #[test]
    fn data_frame_from_floats() {
        let ds: DataSource =
            vec![vec![1.0f32, 2.0f32, 3.0f32], vec![4.0f32, 5.0f32, 6.0f32]].into();
        assert_eq!(
            ds,
            DataSource::Floats(vec![
                vec![1.0f64, 2.0f64, 3.0f64],
                vec![4.0f64, 5.0f64, 6.0f64]
            ])
        );
    }

    #[test]
    fn data_frame_from_bigfloats() {
        let ds: DataSource =
            vec![vec![1.0f64, 2.0f64, 3.0f64], vec![4.0f64, 5.0f64, 6.0f64]].into();
        assert_eq!(
            ds,
            DataSource::Floats(vec![
                vec![1.0f64, 2.0f64, 3.0f64],
                vec![4.0f64, 5.0f64, 6.0f64]
            ])
        );
    }

    #[test]
    fn data_frame_from_mixed() {
        let ds = ds!([1i32, "Tuesday", 3.0f32], ["Monday", 2i32, "Wednesday"]);
        assert_eq!(
            ds,
            DataSource::Mixed(vec![
                vec![
                    CompositeValue::Number(NumericValue::Integer(1)),
                    CompositeValue::String("Tuesday".to_string()),
                    CompositeValue::Number(NumericValue::Float(3.0))
                ],
                vec![
                    CompositeValue::String("Monday".to_string()),
                    CompositeValue::Number(NumericValue::Integer(2)),
                    CompositeValue::String("Wednesday".to_string())
                ],
            ])
        );
    }
}
