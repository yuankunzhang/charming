#![allow(clippy::large_enum_variant)]

use serde::{Deserialize, Deserializer, Serialize};

pub mod bar;
pub mod bar3d;
pub mod boxplot;
pub mod candlestick;
pub mod custom;
pub mod effect_scatter;
pub mod funnel;
pub mod gauge;
pub mod graph;
pub mod heatmap;
pub mod line;
pub mod lines;
pub mod map;
pub mod parallel;
pub mod pictorial_bar;
pub mod pie;
pub mod radar;
pub mod sankey;
pub mod scatter;
pub mod sunburst;
pub mod theme_river;
pub mod tree;
pub mod treemap;

pub use bar::*;
pub use bar3d::*;
pub use boxplot::*;
pub use candlestick::*;
pub use custom::*;
pub use effect_scatter::*;
pub use funnel::*;
pub use gauge::*;
pub use graph::*;
pub use heatmap::*;
pub use line::*;
pub use lines::*;
pub use map::*;
pub use parallel::*;
pub use pictorial_bar::*;
pub use pie::*;
pub use radar::*;
pub use sankey::*;
pub use scatter::*;
pub use sunburst::*;
pub use theme_river::*;
pub use tree::*;
pub use treemap::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Series {
    Bar(bar::Bar),
    Bar3d(bar3d::Bar3d),
    Boxplot(boxplot::Boxplot),
    Candlestick(candlestick::Candlestick),
    Custom(custom::Custom),
    EffectScatter(effect_scatter::EffectScatter),
    Funnel(funnel::Funnel),
    Gauge(gauge::Gauge),
    Graph(graph::Graph),
    Heatmap(heatmap::Heatmap),
    Line(line::Line),
    Map(map::Map),
    Parallel(parallel::Parallel),
    PictorialBar(pictorial_bar::PictorialBar),
    Pie(pie::Pie),
    Radar(radar::Radar),
    Sankey(sankey::Sankey),
    Scatter(scatter::Scatter),
    Sunburst(sunburst::Sunburst),
    ThemeRiver(theme_river::ThemeRiver),
    Tree(tree::Tree),
    Treemap(treemap::Treemap),
}

impl<'de> Deserialize<'de> for Series {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        let result = match value.get("type").and_then(serde_json::Value::as_str) {
            Some(type_) => match type_ {
                "bar" => serde_json::from_value(value).map(Series::Bar),
                "bar3d" => serde_json::from_value(value).map(Series::Bar3d),
                "boxplot" => serde_json::from_value(value).map(Series::Boxplot),
                "candlestick" => serde_json::from_value(value).map(Series::Candlestick),
                "custom" => serde_json::from_value(value).map(Series::Custom),
                "effectScatter" => serde_json::from_value(value).map(Series::EffectScatter),
                "funnel" => serde_json::from_value(value).map(Series::Funnel),
                "gauge" => serde_json::from_value(value).map(Series::Gauge),
                "graph" => serde_json::from_value(value).map(Series::Graph),
                "heatmap" => serde_json::from_value(value).map(Series::Heatmap),
                "line" => serde_json::from_value(value).map(Series::Line),
                "map" => serde_json::from_value(value).map(Series::Map),
                "parallel" => serde_json::from_value(value).map(Series::Parallel),
                "pictorialBar" => serde_json::from_value(value).map(Series::PictorialBar),
                "pie" => serde_json::from_value(value).map(Series::Pie),
                "radar" => serde_json::from_value(value).map(Series::Radar),
                "sankey" => serde_json::from_value(value).map(Series::Sankey),
                "scatter" => serde_json::from_value(value).map(Series::Scatter),
                "sunburst" => serde_json::from_value(value).map(Series::Sunburst),
                "themeRiver" => serde_json::from_value(value).map(Series::ThemeRiver),
                "tree" => serde_json::from_value(value).map(Series::Tree),
                "treemap" => serde_json::from_value(value).map(Series::Treemap),
                unknown => Err(serde::de::Error::custom(format!(
                    "unknown series type: {}",
                    unknown
                ))),
            },
            None => Err(serde::de::Error::custom("missing series type")),
        };

        Ok(result.map_err(serde::de::Error::custom)?)
    }
}

macro_rules! impl_series {
    ($($variant:ident),*) => {
        impl Serialize for Series {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                match self {
                    $(Self::$variant(series) => series.serialize(serializer),)*
                }
            }
        }
        $(
            impl From<$variant> for Series {
                fn from(series: $variant) -> Self {
                    Self::$variant(series)
                }
            }
        )*
    }
}

impl_series!(
    Bar,
    Bar3d,
    Boxplot,
    Custom,
    Candlestick,
    EffectScatter,
    Funnel,
    Gauge,
    Graph,
    Heatmap,
    Line,
    Map,
    Parallel,
    PictorialBar,
    Pie,
    Radar,
    Sankey,
    Scatter,
    Sunburst,
    ThemeRiver,
    Tree,
    Treemap
);
