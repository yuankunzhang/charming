pub enum Theme {
    Default,
    Dark,
    Vintage,
    Westeros,
    Essos,
    Wonderland,
    Walden,
    Chalk,
    Infographic,
    Macarons,
    Roma,
    Shine,
    PurplePassion,
    Halloween,
    Custom(&'static str, &'static str),
}

impl Theme {
    pub(crate) fn to_str(&self) -> (&'static str, &'static str) {
        match self {
            Theme::Default => ("", ""),
            Theme::Dark => ("dark", ""),
            Theme::Vintage => ("vintage", include_str!("../asset/theme-vintage.js")),
            Theme::Westeros => ("westeros", include_str!("../asset/theme-westeros.js")),
            Theme::Essos => ("essos", include_str!("../asset/theme-essos.js")),
            Theme::Wonderland => ("wonderland", include_str!("../asset/theme-wonderland.js")),
            Theme::Walden => ("walden", include_str!("../asset/theme-walden.js")),
            Theme::Chalk => ("chalk", include_str!("../asset/theme-chalk.js")),
            Theme::Infographic => ("infographic", include_str!("../asset/theme-infographic.js")),
            Theme::Macarons => ("macarons", include_str!("../asset/theme-macarons.js")),
            Theme::Roma => ("roma", include_str!("../asset/theme-roma.js")),
            Theme::Shine => ("shine", include_str!("../asset/theme-shine.js")),
            Theme::PurplePassion => (
                "purple-passion",
                include_str!("../asset/theme-purple-passion.js"),
            ),
            Theme::Halloween => ("halloween", include_str!("../asset/theme-halloween.js")),
            Theme::Custom(name, content) => (name, content),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Default
    }
}
