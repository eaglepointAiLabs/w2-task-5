mod api;
mod auth;
mod components;
mod forms;
mod hooks;
mod layouts;
mod pages;
mod theme;
mod types;

use dioxus::prelude::*;
use pages::login::Login;
use pages::dashboard::Dashboard;
use pages::courses::Courses;
use pages::course_detail::CourseDetail;
use pages::course_editor::CourseEditor;
use pages::approvals::Approvals;
use pages::bookings::Bookings;
use pages::risk::Risk;
use pages::privacy::Privacy;
use pages::audit::Audit;

fn main() {
    tracing_wasm::set_as_global_default();
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        components::ToastContainer {}
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(layouts::MainLayout)]
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/courses")]
    Courses {},
    #[route("/courses/:uuid")]
    CourseDetail { uuid: String },
    #[route("/courses/:uuid/edit")]
    CourseEditor { uuid: String },
    #[route("/approvals")]
    Approvals {},
    #[route("/bookings")]
    Bookings {},
    #[route("/risk")]
    Risk {},
    #[route("/privacy")]
    Privacy {},
    #[route("/audit")]
    Audit {},
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
fn Home() -> Element {
    let nav = navigator();
    if api::is_logged_in() {
        nav.replace(Route::Dashboard {});
    } else {
        nav.replace(Route::Login {});
    }
    rsx! {
        div { class: "loading-container",
            p { "Redirecting..." }
        }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "not-found-page",
            div { class: "not-found-content",
                h1 { "404" }
                p { "Page not found." }
                Link { to: Route::Dashboard {}, class: "btn btn-primary", "Go to Dashboard" }
            }
        }
    }
}
