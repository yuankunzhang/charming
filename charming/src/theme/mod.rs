#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Theme {
    #[default]
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

impl TryFrom<&str> for Theme {
    type Error = String;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name {
            "" => Ok(Theme::Default),
            "dark" => Ok(Theme::Dark),
            "vintage" => Ok(Theme::Vintage),
            "westeros" => Ok(Theme::Westeros),
            "essos" => Ok(Theme::Essos),
            "wonderland" => Ok(Theme::Wonderland),
            "walden" => Ok(Theme::Walden),
            "chalk" => Ok(Theme::Chalk),
            "infographic" => Ok(Theme::Infographic),
            "macarons" => Ok(Theme::Macarons),
            "roma" => Ok(Theme::Roma),
            "shine" => Ok(Theme::Shine),
            "purple-passion" => Ok(Theme::PurplePassion),
            "halloween" => Ok(Theme::Halloween),
            _ => Err(format!("unrecognized theme {name:?}")),
        }
    }
}
