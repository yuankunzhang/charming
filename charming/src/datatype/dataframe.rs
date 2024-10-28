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

/// Code below is modified (using df! as reference) from ChatGPT generated code.
/// The main objective of macro dz! is to transpose mixed data type vectors,
/// aka columns or dimensions into ECharts dataframe format.
#[macro_export]
macro_rules! dz {
    /*
    Handle named vectors
    let d1 = vec![44056, 13334];
    let d2 = vec![81.8, 76.9];
    let d3 = vec![23968973, 1376048943];
    let d4 = vec!["Australia", "China"];
    let d5 = vec![2015, 2015];
    let df = dz!(d1, d2, d3, d4, d5);
    >>df = [Value(Array([Number(Integer(44056)), Number(Float(81.8)),
    Number(Integer(23968973)), String("Australia"), Number(Integer(2015))])),
    Value(Array([Number(Integer(13334)), Number(Float(76.9)),
    Number(Integer(1376048943)), String("China"), Number(Integer(2015))]))]
    */
    ($($v:expr),* $(,)?) => {{
        let mut df = Vec::new();
        let mut iterators: Vec<Box<dyn Iterator<Item = $crate::datatype::CompositeValue>>> = vec![
            $(Box::new($v.into_iter().map(|x| $crate::datatype::CompositeValue::from(x))),)*
        ];

        let len = iterators.first().map(|v| v.size_hint().0).unwrap_or(0);

        for _ in 0..len {
            let mut row = Vec::new();
            for iter in iterators.iter_mut() {
                if let Some(value) = iter.next() {
                    row.push(value);
                }
            }
            let z = $crate::datatype::DataPoint::from(row);
            df.push(z);
        }
        df
    }};

    /*
    Handle direct input of vectors
    let df = dz!([44056, 13334], [81.8, 76.9], [23968973, 1376048943], ["Australia", "China"], [2015, 2015]);
    >>df = [Value(Array([Number(Integer(44056)), Number(Float(81.8)),
    Number(Integer(23968973)), String("Australia"), Number(Integer(2015))])),
    Value(Array([Number(Integer(13334)), Number(Float(76.9)),
    Number(Integer(1376048943)), String("China"), Number(Integer(2015))]))]
    */
    ($([$($v:expr),*]),* $(,)?) => {{
        let mut df = Vec::new();
        let mut iterators: Vec<_> = vec![$(vec![$($v.into()),*].into_iter()),*];

        let len = iterators.first().map(|v| v.len()).unwrap_or(0);
        for _ in 0..len {
            let mut row = Vec::new();
            for iter in &mut iterators {
                if let Some($crate::datatype::CompositeValue) = iter.next() {
                    row.push($crate::datatype::CompositeValue);
                }
            }
            df.push(row);
        }
        df
    }};
}
