use serde::Serialize;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum NumericValue {
    Integer(i32),
    Float(f64),
}

impl From<i32> for NumericValue {
    fn from(n: i32) -> Self {
        NumericValue::Integer(n)
    }
}

impl From<f64> for NumericValue {
    fn from(n: f64) -> Self {
        NumericValue::Float(n)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum CompositeValue<'a> {
    Number(NumericValue),
    String(&'a str),
    Array(Vec<CompositeValue<'a>>),
}

impl<N> From<N> for CompositeValue<'static>
where
    N: Into<NumericValue>,
{
    fn from(n: N) -> Self {
        CompositeValue::Number(n.into())
    }
}

impl<'a> From<&'a str> for CompositeValue<'a> {
    fn from(s: &'a str) -> Self {
        CompositeValue::String(s)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum DataFrame<'a> {
    Integers(Vec<Vec<i32>>),
    Floats(Vec<Vec<f64>>),
    Mixed(Vec<Vec<CompositeValue<'a>>>),
}

impl From<Vec<Vec<i32>>> for DataFrame<'static> {
    fn from(v: Vec<Vec<i32>>) -> Self {
        DataFrame::Integers(v)
    }
}

impl From<Vec<Vec<f64>>> for DataFrame<'static> {
    fn from(v: Vec<Vec<f64>>) -> Self {
        DataFrame::Floats(v)
    }
}

impl<'a> From<Vec<Vec<CompositeValue<'a>>>> for DataFrame<'a> {
    fn from(v: Vec<Vec<CompositeValue<'a>>>) -> Self {
        DataFrame::Mixed(v)
    }
}

#[macro_export]
macro_rules! df {
    ($([$($x:expr),*]),*) => {
        $crate::dataset::DataFrame::from(vec![
            $(
                vec![
                    $(
                        $crate::dataset::CompositeValue::from($x)
                    ),*
                ]
            ),*
        ])
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn numeric_value_from_i32() {
        let n: NumericValue = 42i32.into();
        assert_eq!(n, NumericValue::Integer(42));
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
        assert_eq!(s, CompositeValue::String("Monday"));
    }

    #[test]
    fn data_frame_from_integers() {
        let df: DataFrame = vec![vec![1, 2, 3], vec![4, 5, 6]].into();
        assert_eq!(df, DataFrame::Integers(vec![vec![1, 2, 3], vec![4, 5, 6]]));
    }

    #[test]
    fn data_frame_from_floats() {
        let df: DataFrame = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]].into();
        assert_eq!(
            df,
            DataFrame::Floats(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]])
        );
    }

    #[test]
    fn data_frame_from_mixed() {
        let df = df!([1, "Tuesday", 3.0], ["Monday", 2, "Wednesday"]);
        assert_eq!(
            df,
            DataFrame::Mixed(vec![
                vec![
                    CompositeValue::Number(NumericValue::Integer(1)),
                    CompositeValue::String("Tuesday"),
                    CompositeValue::Number(NumericValue::Float(3.0))
                ],
                vec![
                    CompositeValue::String("Monday"),
                    CompositeValue::Number(NumericValue::Integer(2)),
                    CompositeValue::String("Wednesday")
                ],
            ])
        );
    }
}
