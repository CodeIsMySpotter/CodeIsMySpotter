use axum::{response::Html, routing::get, Router};
use askama::Template;

/// Defines the structure for the `me.html` template.
#[derive(Template)]
#[template(path = "me.html")]
struct MeTemplate;

/// Handler for the `/me` route.
/// Renders the `MeTemplate` into an HTML response.
async fn me_page() -> Html<String> {
    let template = MeTemplate;
    
    // Render to a String using unwrap (safe due to Askama's compile-time checks)
    Html(template.render().unwrap())
}

/// Configures and returns the router for the `me` module.
pub fn router() -> Router {
    Router::new().route("/", get(me_page))
}