use super::DataPoint;

pub type DataFrame = Vec<DataPoint>;

#[macro_export]
macro_rules! df {
    ($([$($x:expr),*]),* $(,)?) => {
        vec![
            $(
                $crate::datatype::DataPoint::from($crate::datatype::CompositeValue::from(vec![
                    $(
                        $crate::datatype::CompositeValue::from($x)
                    ),*
                ]))
            ),*
        ]
    };
    ($($x:expr),* $(,)?) => {
        vec![
            $(
                $crate::datatype::DataPoint::from($x)
            ),*
        ]
    };
}
