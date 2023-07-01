use serde_json::Value;

pub type DataPoint = Vec<Value>;

pub type DataFrameOneDimension = Vec<Value>;

pub type DataFrame = Vec<DataPoint>;
