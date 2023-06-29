use std::error::Error;

pub trait Series {
    fn to_json_string(&self) -> Result<String, Box<dyn Error>> {
        Err("Need to implement Series::build".into())
    }
}

pub struct DummySeries {}

impl Series for DummySeries {}

pub struct LineSeries {
    pub data: Vec<Vec<f64>>,
}

impl Series for LineSeries {
    fn to_json_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(serde_json::to_string(&self.data)?)
    }
}

pub struct BarSeries {}

impl Series for BarSeries {}

pub struct PieSeries {}

impl Series for PieSeries {}

pub struct ScatterSeries {}

impl Series for ScatterSeries {}

pub struct EffectScatterSeries {}

impl Series for EffectScatterSeries {}

pub struct RadarSeries {}

impl Series for RadarSeries {}

pub struct TreeSeries {}

impl Series for TreeSeries {}

pub struct TreemapSeries {}

impl Series for TreemapSeries {}

pub struct SunburstSeries {}

impl Series for SunburstSeries {}

pub struct BoxplotSeries {}

impl Series for BoxplotSeries {}

pub struct CandlestickSeries {}

impl Series for CandlestickSeries {}

pub struct HeatmapSeries {}

impl Series for HeatmapSeries {}

pub struct MapSeries {}

impl Series for MapSeries {}

pub struct ParallelSeries {}

impl Series for ParallelSeries {}

pub struct LinesSeries {}

impl Series for LinesSeries {}

pub struct GraphSeries {}

impl Series for GraphSeries {}

pub struct SankeySeries {}

impl Series for SankeySeries {}

pub struct FunnelSeries {}

impl Series for FunnelSeries {}

pub struct GaugeSeries {}

impl Series for GaugeSeries {}

pub struct PictorialBarSeries {}

impl Series for PictorialBarSeries {}

pub struct ThemeRiverSeries {}

impl Series for ThemeRiverSeries {}

pub struct CustomSeries {}

impl Series for CustomSeries {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_series() {
        let s = LineSeries {
            data: vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
        };

        assert_eq!(s.to_json_string().unwrap(), "[[1.0,2.0,3.0],[4.0,5.0,6.0]]")
    }
}
