use std::collections::BTreeMap;

use askama::Template;
use axum::{
    extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use echarts::{Chart, HtmlRenderer};
use lazy_static::lazy_static;

mod bar;
mod bar3d;
mod boxplot;
mod candlestick;
mod dataset;
mod funnel;
mod gauge;
mod geo;
mod graph;
mod heatmap;
mod line;
mod parallel;
mod pictorial_bar;
mod pie;
mod radar;
mod sankey;
mod scatter;
mod sunburst;
mod theme_river;
mod tree;
mod treemap;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/:type/:name", get(render));

    axum::Server::bind(&"127.0.0.1:5555".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

macro_rules! insert {
    ($map:ident, $type:ident, $name:ident) => {
        $map.insert(stringify!($name), $type::$name::chart as fn() -> Chart);
    };
}

lazy_static! {
    static ref BAR_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, bar, bar_with_background);
        insert!(m, bar, stacked_column);
        insert!(m, bar, tangential_polar_bar);
        insert!(m, bar, waterfall);
        m
    };
    static ref BOXPLOT_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, boxplot, boxplot_light_velocity);
        insert!(m, boxplot, multiple_categories);
        m
    };
    static ref CANDLESTICK_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, candlestick, basic_candlestick);
        m
    };
    static ref DATASET_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, dataset, encode_and_matrix);
        m
    };
    static ref FUNNEL_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, funnel, multiple_funnels);
        m
    };
    static ref GAUGE_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, gauge, gauge_barometer);
        m
    };
    static ref GEO_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, geo, organ_data);
        m
    };
    static ref GRAPH_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, graph, hide_overlapped_label);
        insert!(m, graph, les_miserables);
        m
    };
    static ref HEATMAP_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, heatmap, heatmap_on_cartesian);
        m
    };
    static ref LINE_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, line, area_pieces);
        insert!(m, line, basic_area);
        insert!(m, line, basic_line);
        insert!(m, line, data_transform_filter);
        insert!(m, line, distribution_of_electricity);
        insert!(m, line, gradient_stacked_area);
        insert!(m, line, large_scale_area);
        insert!(m, line, line_gradient);
        insert!(m, line, smoothed_line);
        insert!(m, line, stacked_area);
        insert!(m, line, stacked_line);
        insert!(m, line, temperature_change);
        m
    };
    static ref PARALLEL_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, parallel, basic_parallel);
        insert!(m, parallel, parallel_aqi);
        m
    };
    static ref PICTORIAL_BAR_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, pictorial_bar, water_content);
        m
    };
    static ref PIE_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, pie, nightingale);
        m
    };
    static ref RADAR_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, radar, multiple_radar);
        insert!(m, radar, proportion_of_browsers);
        m
    };
    static ref SANKEY_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, sankey, node_align_left_sankey);
        m
    };
    static ref SCATTER_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, scatter, basic_scatter);
        insert!(m, scatter, effect_scatter);
        insert!(m, scatter, punch_card_of_github);
        m
    };
    static ref SUNBURST_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, sunburst, drink_flavors);
        m
    };
    static ref THEME_RIVER_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, theme_river, theme_river_lastfm);
        m
    };
    static ref TREE_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, tree, from_left_to_right_tree);
        m
    };
    static ref TREEMAP_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, treemap, disk_usage);
        m
    };
    static ref CHARTS: BTreeMap<&'static str, BTreeMap<&'static str, fn() -> Chart>> = {
        let mut m = BTreeMap::new();
        m.insert("bar", BAR_CHARTS.clone());
        m.insert("boxplot", BOXPLOT_CHARTS.clone());
        m.insert("candlestick", CANDLESTICK_CHARTS.clone());
        m.insert("dataset", DATASET_CHARTS.clone());
        m.insert("funnel", FUNNEL_CHARTS.clone());
        m.insert("gauge", GAUGE_CHARTS.clone());
        m.insert("geo", GEO_CHARTS.clone());
        m.insert("graph", GRAPH_CHARTS.clone());
        m.insert("heatmap", HEATMAP_CHARTS.clone());
        m.insert("line", LINE_CHARTS.clone());
        m.insert("parallel", PARALLEL_CHARTS.clone());
        m.insert("pictorial_bar", PICTORIAL_BAR_CHARTS.clone());
        m.insert("pie", PIE_CHARTS.clone());
        m.insert("radar", RADAR_CHARTS.clone());
        m.insert("sankey", SANKEY_CHARTS.clone());
        m.insert("scatter", SCATTER_CHARTS.clone());
        m.insert("sunburst", SUNBURST_CHARTS.clone());
        m.insert("theme_river", THEME_RIVER_CHARTS.clone());
        m.insert("tree", TREE_CHARTS.clone());
        m.insert("treemap", TREEMAP_CHARTS.clone());
        m
    };
}

async fn index() -> impl IntoResponse {
    let mut template = IndexTemplate::new();
    for (key, value) in CHARTS.iter() {
        template.collection(key, value.iter().map(|(k, _)| *k).collect::<Vec<_>>());
    }
    HtmlTemplate(template)
}

async fn render(
    extract::Path((r#type, name)): extract::Path<(String, String)>,
) -> impl IntoResponse {
    let renderer = HtmlRenderer::new(format!("{type} - {name}"), 1000, 800);

    let chart = match CHARTS.get(r#type.as_str()) {
        Some(charts) => match charts.get(name.as_str()) {
            Some(chart) => chart(),
            None => return (StatusCode::NOT_FOUND, "Chart Not Found").into_response(),
        },
        None => return (StatusCode::NOT_FOUND, "Chart Type Not Found").into_response(),
    };
    Html(renderer.render(chart)).into_response()
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    collections: Vec<(String, Vec<String>)>,
}

impl IndexTemplate {
    fn new() -> Self {
        Self {
            collections: vec![],
        }
    }

    fn collection(&mut self, name: &str, charts: Vec<&str>) {
        self.collections.push((
            name.to_string(),
            charts.into_iter().map(|s| s.to_string()).collect(),
        ));
    }
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(body) => Html(body).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Template error: {}", e),
            )
                .into_response(),
        }
    }
}
