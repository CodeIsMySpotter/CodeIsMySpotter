use axum::{response::Html, routing::get, Router};
use askama::Template;

/// 1. Defines 3 classes of project tiles using an enum.
#[derive(Clone)]
pub enum ProjectStatus {
    Concept,
    Forging,
    /// The `Ready` status contains a required URL link.
    Ready { link: &'static str },
}

/// 2. Structure representing a single project.
#[derive(Clone)]
pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    /// Emoji or icon name
    pub icon: &'static str,
    pub status: ProjectStatus,
}

/// 3. Askama template that will receive a vector of our projects.
#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate {
    projects: Vec<Project>,
}

/// Handler for the `/projects` route.
/// Renders the `ProjectsTemplate` with a list of projects into an HTML response.
async fn projects_page() -> Html<String> {
    // Define your portfolio here. You can freely modify it!
    let projects = vec![
        Project {
            title: "Fundly",
            description: "A high-performance stock screener tailored for fundamental analysis and historical data backtesting.",
            icon: "📊",
            status: ProjectStatus::Forging, // Status: Forging 🔨
        },
        Project {
            title: "Stormy",
            description: "A live collaborative whiteboard platform for students to exchange notes and study material in real-time.",
            icon: "⚡",
            status: ProjectStatus::Forging, // Status: Forging 🔨
        },
        Project {
            title: "Live Portfolio",
            description: "My personal, minimalist space on the web built with Axum, Askama, and HTMX. Hosted on Fly.io.",
            icon: "🥷",
            status: ProjectStatus::Ready { link: "https://spotter-portfolio.fly.dev" }, // Status: Ready ✨
        },
    ];

    let template = ProjectsTemplate { projects };
    Html(template.render().unwrap())
}

/// Configures and returns the router for the `projects` module.
pub fn router() -> Router {
    Router::new().route("/", get(projects_page))
}