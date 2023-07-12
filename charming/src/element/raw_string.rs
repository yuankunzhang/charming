use serde::Serialize;

pub static RAW_MARK: &str = "#*#*#*#";

#[derive(Debug)]
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

pub(crate) fn process_raw_strings(s: &str) -> String {
    let left_mark = format!("\"{}", RAW_MARK);
    let right_mark = format!("{}\"", RAW_MARK);

    let mut output = String::with_capacity(s.len());
    let mut pos = 0;

    while pos < s.len() {
        let left = pos + s[pos..].find(&left_mark).unwrap_or_else(|| s.len() - pos);
        output.push_str(&s[pos..left]);

        if left >= s.len() {
            break;
        }

        pos = left + left_mark.len();
        let right = pos + s[pos..].find(&right_mark).unwrap_or_else(|| s.len() - pos);
        output.push_str(&unescape_string(&s[pos..right]));

        pos = right + right_mark.len();
    }

    output
}

fn unescape_string(s: &str) -> String {
    let mut unescaped = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' && chars.peek().is_some() {
            unescaped.push(match chars.next().unwrap() {
                '\\' => '\\',
                '"' => '"',
                '\'' => '\'',
                '/' => '/',
                'b' => '\x08',
                'f' => '\x0c',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                'v' => '\x0b',
                c => c,
            })
        } else {
            unescaped.push(c);
        }
    }

    unescaped
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn raw_strings() {
        let s = format!("\"{}foobar{}\"", RAW_MARK, RAW_MARK);
        assert_eq!(process_raw_strings(&s), "foobar");

        let s = format!("foo\"{}bar{}\"baz", RAW_MARK, RAW_MARK);
        assert_eq!(process_raw_strings(&s), "foobarbaz");

        let s = format!("foo\"{}b\\na\\nr{}\"baz", RAW_MARK, RAW_MARK);
        assert_eq!(process_raw_strings(&s), "foob\na\nrbaz");
    }
}
