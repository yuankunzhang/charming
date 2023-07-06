use serde::Serialize;

pub mod bar;
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
    Line(line::Line),
    Bar(bar::Bar),
    Pie(pie::Pie),
    Scatter(scatter::Scatter),
    EffectScatter(effect_scatter::EffectScatter),
    Radar(radar::Radar),
    Tree(tree::Tree),
    Treemap(treemap::Treemap),
    Sunburst(sunburst::Sunburst),
    Boxplot(boxplot::Boxplot),
    Candlestick(candlestick::Candlestick),
    Heatmap(heatmap::Heatmap),
    Map(map::Map),
    Parallel(parallel::Parallel),
    Graph(graph::Graph),
    Sankey(sankey::Sankey),
    Funnel(funnel::Funnel),
    Gauge(gauge::Gauge),
    PictorialBar(pictorial_bar::PictorialBar),
    ThemeRiver(theme_river::ThemeRiver),
    Custom(custom::Custom),
}

macro_rules! serialize {
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
    }
}

serialize!(
    Line,
    Bar,
    Pie,
    Scatter,
    EffectScatter,
    Radar,
    Tree,
    Treemap,
    Sunburst,
    Boxplot,
    Candlestick,
    Heatmap,
    Map,
    Parallel,
    Graph,
    Sankey,
    Funnel,
    Gauge,
    PictorialBar,
    ThemeRiver,
    Custom
);

macro_rules! impl_series {
    ($($variant:ident),*) => {
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
    Line,
    Bar,
    Pie,
    Scatter,
    EffectScatter,
    Radar,
    Tree,
    Treemap,
    Sunburst,
    Boxplot,
    Candlestick,
    Heatmap,
    Map,
    Parallel,
    Graph,
    Sankey,
    Funnel,
    Gauge,
    PictorialBar,
    ThemeRiver,
    Custom
);
