use serde::Serialize;

pub static RAW_MARK: &str = "#*#*#*#";

pub struct RawString(String);

impl Serialize for RawString {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(format!("{}{}{}", RAW_MARK, self.0, RAW_MARK).as_str())
    }
}

impl<S> From<S> for RawString
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        RawString(s.into())
    }
}
