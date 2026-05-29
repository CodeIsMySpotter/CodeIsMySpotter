use axum::{response::Html, routing::get, Router};
use askama::Template;

// 1. Definiujemy 3 klasy kafelków za pomocą enuma
#[derive(Clone)]
pub enum ProjectStatus {
    Concept,
    Forging,
    Ready { link: &'static str }, // Klasa Ready zawiera w sobie wymagany link
}

// 2. Struktura reprezentująca pojedynczy projekt
#[derive(Clone)]
pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    pub icon: &'static str, // Emoji lub nazwa ikonki
    pub status: ProjectStatus,
}

// 3. Szablon Askamy, do którego przekażemy wektor naszych projektów
#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate {
    projects: Vec<Project>,
}

async fn projects_page() -> Html<String> {
    // Tutaj definiujesz swoje portfolio. Możesz je teraz dowolnie modyfikować!
    let projects = vec![
        Project {
            title: "Fundly",
            description: "A high-performance stock screener tailored for fundamental analysis and historical data backtesting.",
            icon: "📊",
            status: ProjectStatus::Forging, // Klasa: W kuźni 🔨
        },
        Project {
            title: "Stormy",
            description: "A live collaborative whiteboard platform for students to exchange notes and study material in real-time.",
            icon: "⚡",
            status: ProjectStatus::Concept, // Klasa: Koncepcja 💡
        },
        Project {
            title: "Live Portfolio",
            description: "My personal, minimalist space on the web built with Axum, Askama, and HTMX. Hosted on Fly.io.",
            icon: "🥷",
            status: ProjectStatus::Ready { link: "https://spotter-portfolio.fly.dev" }, // Klasa: Gotowy ✨
        },
    ];

    let template = ProjectsTemplate { projects };
    Html(template.render().unwrap())
}

pub fn router() -> Router {
    Router::new().route("/", get(projects_page))
}