use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum GeoMapOpt {
    #[serde(rename = "geoJSON")]
    GeoJson {
        value: serde_json::Value,
        special_areas: serde_json::Value,
    },
    #[serde(rename = "svg")]
    Svg(String),
}

impl<S> From<S> for GeoMapOpt
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        GeoMapOpt::Svg(s.into())
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoMap {
    name: Option<String>,
    opt: Option<GeoMapOpt>,
}

impl From<&str> for GeoMap {
    fn from(svg: &str) -> Self {
        GeoMap::new().opt(GeoMapOpt::Svg(svg.to_string()))
    }
}

impl From<(&str, &str)> for GeoMap {
    fn from((name, svg): (&str, &str)) -> Self {
        GeoMap::new()
            .name(name.to_string())
            .opt(GeoMapOpt::Svg(svg.to_string()))
    }
}
