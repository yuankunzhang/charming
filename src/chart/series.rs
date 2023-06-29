use serde::Serialize;

pub enum Series {
    Line(Line),
}

impl Serialize for Series {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Series::Line(series) => series.serialize(serializer),
        }
    }
}

#[derive(Serialize)]
pub struct Line {
    #[serde(rename = "type")]
    type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    smooth: bool,
    data: Vec<Vec<f64>>,
}

impl Line {
    pub fn new() -> Self {
        Self {
            type_: "line".to_string(),
            name: None,
            smooth: false,
            data: vec![],
        }
    }

    /// Series name used for displaying in `tooltip` and filtering with `legend`.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Whether to show as smooth curve.
    pub fn smooth(mut self, smooth: bool) -> Self {
        self.smooth = smooth;
        self
    }

    /// Data array of series.
    pub fn data(mut self, data: Vec<Vec<f64>>) -> Self {
        self.data = data;
        self
    }
}

#[derive(Serialize)]
pub struct BarSeries {}

#[derive(Serialize)]
pub struct PieSeries {}

#[derive(Serialize)]
pub struct ScatterSeries {}

#[derive(Serialize)]
pub struct EffectScatterSeries {}

#[derive(Serialize)]
pub struct RadarSeries {}

#[derive(Serialize)]
pub struct TreeSeries {}

#[derive(Serialize)]
pub struct TreemapSeries {}

#[derive(Serialize)]
pub struct SunburstSeries {}

#[derive(Serialize)]
pub struct BoxplotSeries {}

#[derive(Serialize)]
pub struct CandlestickSeries {}

#[derive(Serialize)]
pub struct HeatmapSeries {}

#[derive(Serialize)]
pub struct MapSeries {}

#[derive(Serialize)]
pub struct ParallelSeries {}

#[derive(Serialize)]
pub struct LinesSeries {}

#[derive(Serialize)]
pub struct GraphSeries {}

#[derive(Serialize)]
pub struct SankeySeries {}

#[derive(Serialize)]
pub struct FunnelSeries {}

#[derive(Serialize)]
pub struct GaugeSeries {}

#[derive(Serialize)]
pub struct PictorialBarSeries {}

#[derive(Serialize)]
pub struct ThemeRiverSeries {}

#[derive(Serialize)]
pub struct CustomSeries {}
