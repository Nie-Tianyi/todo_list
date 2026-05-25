use crate::auth::AuthContext;
use crate::backend::register;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Register() -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let auth = use_context::<AuthContext>();
    let navigator = use_navigator();

    let authenticated = auth.is_authenticated();

    use_effect(move || {
        if authenticated {
            navigator.push(Route::Todos {});
        }
    });

    if authenticated {
        return rsx! { div {} };
    }

    rsx! {
        div { class: "min-h-screen flex items-center justify-center bg-gray-50",
            div { class: "max-w-md w-full mx-4",
                div { class: "bg-white rounded-2xl shadow-sm border border-gray-100 p-8",
                    h2 { class: "text-2xl font-bold text-gray-800 mb-2", "Create Account" }
                    p { class: "text-sm text-gray-500 mb-6", "Sign up to start managing your tasks." }

                    if let Some(ref msg) = error() {
                        div { class: "mb-4 p-3 bg-red-50 border border-red-100 text-red-600 text-sm rounded-lg",
                            "{msg}"
                        }
                    }

                    div { class: "mb-4",
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1.5",
                            r#for: "username",
                            "Username"
                        }
                        input {
                            id: "username",
                            r#type: "text",
                            class: "w-full border border-gray-200 rounded-lg px-3 py-2.5 text-sm
                                    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                    transition-shadow",
                            placeholder: "Choose a username",
                            value: "{username}",
                            oninput: move |e| {
                                username.set(e.value());
                                error.set(None);
                            },
                        }
                    }

                    div { class: "mb-4",
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1.5",
                            r#for: "password",
                            "Password"
                        }
                        input {
                            id: "password",
                            r#type: "password",
                            class: "w-full border border-gray-200 rounded-lg px-3 py-2.5 text-sm
                                    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                    transition-shadow",
                            placeholder: "At least 6 characters",
                            value: "{password}",
                            oninput: move |e| {
                                password.set(e.value());
                                error.set(None);
                            },
                        }
                    }

                    div { class: "mb-6",
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1.5",
                            r#for: "confirm",
                            "Confirm Password"
                        }
                        input {
                            id: "confirm",
                            r#type: "password",
                            class: "w-full border border-gray-200 rounded-lg px-3 py-2.5 text-sm
                                    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                    transition-shadow",
                            placeholder: "Repeat your password",
                            value: "{confirm}",
                            oninput: move |e| {
                                confirm.set(e.value());
                                error.set(None);
                            },
                            onkeydown: move |e| {
                                if e.key() == Key::Enter && !loading() {
                                    let u = username().trim().to_string();
                                    let p = password().trim().to_string();
                                    let cp = confirm().trim().to_string();
                                    if u.is_empty() || p.is_empty() || cp.is_empty() {
                                        error.set(Some("Please fill in all fields.".into()));
                                        return;
                                    }
                                    if p != cp {
                                        error.set(Some("Passwords do not match.".into()));
                                        return;
                                    }
                                    if p.len() < 6 {
                                        error.set(Some("Password must be at least 6 characters.".into()));
                                        return;
                                    }
                                    loading.set(true);
                                    error.set(None);
                                    spawn(async move {
                                        match register(u, p).await {
                                            Ok(resp) => {
                                                auth.login(crate::auth::AuthState {
                                                    user: resp.user,
                                                    token: resp.token,
                                                });
                                                navigator.push(Route::Todos {});
                                            }
                                            Err(e) => {
                                                error.set(Some(e.to_string()));
                                                loading.set(false);
                                            }
                                        }
                                    });
                                }
                            },
                        }
                    }

                    button {
                        class: "w-full px-4 py-2.5 bg-blue-500 text-white text-sm font-medium rounded-lg
                                hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
                                disabled:opacity-50 disabled:cursor-not-allowed transition-colors",
                        disabled: loading(),
                        onclick: move |_| {
                            let u = username().trim().to_string();
                            let p = password().trim().to_string();
                            let cp = confirm().trim().to_string();
                            if u.is_empty() || p.is_empty() || cp.is_empty() {
                                error.set(Some("Please fill in all fields.".into()));
                                return;
                            }
                            if p != cp {
                                error.set(Some("Passwords do not match.".into()));
                                return;
                            }
                            if p.len() < 6 {
                                error.set(Some("Password must be at least 6 characters.".into()));
                                return;
                            }
                            loading.set(true);
                            error.set(None);
                            spawn(async move {
                                match register(u, p).await {
                                    Ok(resp) => {
                                        auth.login(crate::auth::AuthState {
                                            user: resp.user,
                                            token: resp.token,
                                        });
                                        navigator.push(Route::Todos {});
                                    }
                                    Err(e) => {
                                        error.set(Some(e.to_string()));
                                        loading.set(false);
                                    }
                                }
                            });
                        },
                        if loading() {
                            "Creating account..."
                        } else {
                            "Create Account"
                        }
                    }

                    p { class: "text-center text-sm text-gray-500 mt-4",
                        "Already have an account? "
                        Link {
                            to: Route::Login {},
                            class: "text-blue-500 hover:text-blue-600 font-medium",
                            "Sign in"
                        }
                    }
                }
            }
        }
    }
}
