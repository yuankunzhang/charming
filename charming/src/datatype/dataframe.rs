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

#[macro_export]
macro_rules! df_transposed {
    ($([$($x:expr),*]),* $(,)?) => {

        // Determine the length of the vectors (assuming they are all the same length)
        let len = $crate::vec_len!($($col),*);

        // Create a vector to store the resulting rows
        let mut rows = Vec::with_capacity(len);

        // Iterate over the length of the vectors
        for i in 0..len {
            vec![
                $(
                    $crate::datatype::DataPoint::from($crate::datatype::CompositeValue::from(vec![
                        $(
                            $crate::datatype::CompositeValue::from($x[i].clone())
                        ),*
                    ]))
                ),*
            ]
        }
    };
    ($($x:expr),* $(,)?) => {
        vec![
            $(
                $crate::datatype::DataPoint::from($x)
            ),*
        ]
    };
}

#[macro_export]
macro_rules! vec_len {
    ($first:expr $(, $rest:expr)*) => {
        $first.len()
    };
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr};

#[proc_macro]
pub fn transpose(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ExprTuple);

    let vectors: Vec<_> = input.elems.iter().map(|expr| {
        match expr {
            syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) => {
                let ident = Ident::new(&s.value(), s.span());
                quote! { #ident }
            }
            _ => panic!("Expected vector identifiers"),
        }
    }).collect();

    let len = vectors.len();
    let first_vec = &vectors[0];

    let expanded = quote! {
        {
            let mut result = Vec::with_capacity(#len);
            for i in 0..#first_vec.len() {
                result.push(vec![
                    #((#vectors[j])[i]),*
                ]);
            }
            result
        }
    };

    expanded.into()
}

