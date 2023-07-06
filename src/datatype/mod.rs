pub mod data_forest;
pub mod data_frame;
pub mod data_point;
pub mod value;

pub use data_forest::*;
pub use data_frame::*;
pub use data_point::*;
pub use value::*;

#[macro_export]
macro_rules! row {
    ($($value:expr),*) => {
        vec![
            $(
                $crate::datatype::value($value)
            ),*]
    };
}
