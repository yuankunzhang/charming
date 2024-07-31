use super::DataPoint;

/// [`DataFrame`] is the basic data representation in Echarts.
///
/// ## DataFrame
///
/// Basically, data in Echarts is represented by a nested array. like the
/// following example, where each column is named as a "dimension".
///
/// data: [
///     // dimX   dimY   other dimensions ...
///     [  3.4,    4.5,   15,   43],
///     [  4.2,    2.3,   20,   91],
///     [  10.8,   9.5,   30,   18],
///     [  7.2,    8.8,   18,   57]
/// ]
///
/// We can use the [`df`] macro to construct a DataFrame. For example, to
/// construct the above DataFrame, you can write code like this:
///
/// ```rust
/// use charming::datatype::DataFrame;
/// use charming::df;
///
/// let data: DataFrame = df![
///    [3.4, 4.5, 15, 43],
///    [4.2, 2.3, 20, 91],
///    [10.8, 9.5, 30, 18],
///    [7.2, 8.8, 18, 57]
/// ];
/// ```
///
/// Especially, when there is only one dimension in each row, data can be
/// simply represented by a plain array, like the following example:
///
/// data: [1, 1, 2, 3, 5, 7, 13]
///
/// We can use the second form of the [`df`] macro to construct the above
/// simplified DataFrame. For example, to construct the above DataFrame, you
/// can write code like this:
///
/// ```rust
/// use charming::datatype::DataFrame;
/// use charming::df;
///
/// let data: DataFrame = df![1, 1, 2, 3, 5, 7, 13];
/// ```
///
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
