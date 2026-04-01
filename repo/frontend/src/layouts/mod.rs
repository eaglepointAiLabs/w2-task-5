use dioxus::prelude::*;
use crate::auth::AUTH;
use crate::api;

#[component]
pub fn MainLayout() -> Element {
    let auth = AUTH.read();

    // Route guard: redirect unauthenticated users to login
    // Allow /login route through (it's inside this layout)
    let is_login_page = web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .map(|p| p == "/login")
        .unwrap_or(false);

    if !auth.is_authenticated && !api::is_logged_in() && !is_login_page {
        let nav = navigator();
        nav.replace(crate::Route::Login {});
        return rsx! {
            div { class: "loading-container",
                p { "Redirecting to login..." }
            }
        };
    }

    rsx! {
        div { class: "app-container",
            if auth.is_authenticated {
                nav { class: "sidebar",
                    div { class: "sidebar-header",
                        h2 { class: "sidebar-title", "CampusLearn" }
                        p { class: "sidebar-subtitle", "Operations Suite" }
                    }
                    div { class: "sidebar-nav",
                        Link { to: crate::Route::Dashboard {}, class: "nav-item", "Dashboard" }

                        div { class: "nav-section-label", "ACADEMIC" }
                        Link { to: crate::Route::Courses {}, class: "nav-item", "Courses" }
                        if auth.role() == "admin" || auth.role() == "dept_reviewer" {
                            Link { to: crate::Route::Approvals {}, class: "nav-item", "Approvals" }
                        }

                        div { class: "nav-section-label", "RESOURCES" }
                        Link { to: crate::Route::Bookings {}, class: "nav-item", "Bookings" }

                        if auth.is_admin() {
                            div { class: "nav-section-label", "ADMIN" }
                            Link { to: crate::Route::Risk {}, class: "nav-item", "Risk & Compliance" }
                            Link { to: crate::Route::Audit {}, class: "nav-item", "Audit Trail" }
                        }

                        div { class: "nav-section-label", "ACCOUNT" }
                        Link { to: crate::Route::Privacy {}, class: "nav-item", "Privacy & Data" }
                    }
                    div { class: "sidebar-footer",
                        if let Some(user) = &auth.user {
                            div { class: "user-info",
                                span { class: "user-name", "{user.full_name}" }
                                span { class: "user-role", "{user.role}" }
                            }
                        }
                        button {
                            class: "btn btn-outline btn-sm btn-full",
                            onclick: move |_| crate::auth::logout(),
                            "Sign Out"
                        }
                    }
                }
            }
            main { class: if auth.is_authenticated { "main-content with-sidebar" } else { "main-content" },
                Outlet::<crate::Route> {}
            }
        }
    }
}
