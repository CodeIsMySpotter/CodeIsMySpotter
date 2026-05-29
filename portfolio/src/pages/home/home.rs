use axum::{routing::get, Router, response::Html};
use askama::Template;

/// Defines the template structure and points to the corresponding file in the `/templates` directory.
#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

/// Handler for the home page.
/// Returns the rendered `HomeTemplate` as an HTML response.
async fn home_page() -> Html<String> {
    Html(HomeTemplate.render().unwrap())
}

/// Configures and returns the router for the `home` module.
pub fn router() -> Router {
    Router::new().route("/", get(home_page))
}