use axum::{response::Html, routing::get, Router};
use askama::Template;

// Definiujemy strukturę dla szablonu me.html
#[derive(Template)]
#[template(path = "me.html")]
struct MeTemplate;

async fn me_page() -> Html<String> {
    let template = MeTemplate;
    
    // Render do Stringa przy użyciu unwrap (bezpieczne dzięki kompilacji Askamy)
    Html(template.render().unwrap())
}

pub fn router() -> Router {
    Router::new().route("/", get(me_page))
}