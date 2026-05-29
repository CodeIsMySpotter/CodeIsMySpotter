use axum::{routing::get, Router, response::Html};
use askama::Template;

// Definiujemy strukturę szablonu i wskazujemy plik w folderze /templates
#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

// Handler zwraca strukturę szablonu
async fn home_page() -> Html<String> {
    Html(HomeTemplate.render().unwrap())
}

pub fn router() -> Router {
    Router::new().route("/", get(home_page))
}