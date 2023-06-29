pub enum Theme {
    Default,
    White,
    Dark,
}

impl ToString for Theme {
    fn to_string(&self) -> String {
        match self {
            Theme::Default | Theme::White => "white".to_string(),
            Theme::Dark => "dark".to_string(),
        }
    }
}

pub enum ColorPalette {
    Default,
    Custom(Vec<String>),
}
