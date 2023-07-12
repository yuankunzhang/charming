use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    map_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    opt: Option<GeoMapOpt>,
}

impl GeoMap {
    pub fn new() -> Self {
        GeoMap {
            map_name: None,
            opt: None,
        }
    }

    pub fn map_name<S: Into<String>>(mut self, map_name: S) -> Self {
        self.map_name = Some(map_name.into());
        self
    }

    pub fn opt<M: Into<GeoMapOpt>>(mut self, opt: M) -> Self {
        self.opt = Some(opt.into());
        self
    }
}

impl From<&str> for GeoMap {
    fn from(svg: &str) -> Self {
        GeoMap::new().opt(GeoMapOpt::Svg(svg.to_string()))
    }
}

impl From<(&str, &str)> for GeoMap {
    fn from((map_name, svg): (&str, &str)) -> Self {
        GeoMap::new()
            .map_name(map_name.to_string())
            .opt(GeoMapOpt::Svg(svg.to_string()))
    }
}
