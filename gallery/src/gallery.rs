use askama::Template;
use axum::{
    extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use charming::HtmlRenderer;
use charming_gallery::CHARTS;

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
    Html(renderer.render(&chart).unwrap()).into_response()
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
