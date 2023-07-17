use serde::Serialize;

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
