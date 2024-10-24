/*
use charming::datatype::DataFrame;
use charming::dz;

use charming::component::Type::Value;
use charming::datatype::DataPoint::Value;
use charming::element::AxisType::Value;
use charming::element::Color::Value;
use charming::datatype::CompositeValue::Array;
use serde_json::Value::Array;
use charming::datatype::CompositeValue::String;
use charming::element::Formatter::String;
use serde_json::Value::String;
use charming::datatype::NumericValue::Integer;
use charming::datatype::CompositeValue::Number;
use charming::datatype::DimensionType::Number;
use charming::element::SymbolSize::Number;
use serde_json::Value::Number;
use charming::datatype::NumericValue::Integer;
use charming::datatype::DimensionType::Float;
use charming::datatype::NumericValue::Float;
*/

#[cfg(test)]
mod tests {

    fn dz_macro_case_1() {
        // Set up test data
        let d1 = vec![44056, 13334];
        let d2 = vec![81.8, 76.9];
        let d3 = vec![23968973, 1376048943];
        let d4 = vec!["Australia", "China"];
        let d5 = vec![2015, 2015];

        // expected output
        let expected_output: DataFrame = [Value(Array([Number(Integer(44056)), Number(Float(81.8)), Number(Integer(23968973)), String("Australia"), Number(Integer(2015))])), Value(Array([Number(Integer(13334)), Number(Float(76.9)), Number(Integer(1376048943)), String("China"), Number(Integer(2015))]))];

        // Call the macro
        let df = dz!(d1, d2, d3, d4, d5);

        // Assert expected output
        assert_eq!(df, expected_output);
    }
}