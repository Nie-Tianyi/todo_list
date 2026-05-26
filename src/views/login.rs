use crate::auth::AuthContext;
use crate::backend::login;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
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
        div { class: "min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-950",
            div { class: "max-w-md w-full mx-4",
                div { class: "bg-white dark:bg-gray-900 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700 p-8",
                    h2 { class: "text-2xl font-bold text-gray-800 dark:text-gray-100 mb-2", "Sign In" }
                    p { class: "text-sm text-gray-500 dark:text-gray-400 mb-6", "Enter your credentials to access the app." }

                    if let Some(ref msg) = error() {
                        div { class: "mb-4 p-3 bg-red-50 dark:bg-red-900/30 border border-red-100 dark:border-red-800 text-red-600 dark:text-red-400 text-sm rounded-lg",
                            "{msg}"
                        }
                    }

                    div { class: "mb-4",
                        label {
                            class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                            r#for: "username",
                            "Username"
                        }
                        input {
                            id: "username",
                            r#type: "text",
                            class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2.5 text-sm dark:text-gray-100 dark:bg-gray-800
                                    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                    transition-shadow",
                            placeholder: "Enter your username",
                            value: "{username}",
                            oninput: move |e| {
                                username.set(e.value());
                                error.set(None);
                            },
                        }
                    }

                    div { class: "mb-6",
                        label {
                            class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                            r#for: "password",
                            "Password"
                        }
                        input {
                            id: "password",
                            r#type: "password",
                            class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2.5 text-sm dark:text-gray-100 dark:bg-gray-800
                                    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                    transition-shadow",
                            placeholder: "Enter your password",
                            value: "{password}",
                            oninput: move |e| {
                                password.set(e.value());
                                error.set(None);
                            },
                            onkeydown: move |e| {
                                if e.key() == Key::Enter && !loading() {
                                    let u = username().trim().to_string();
                                    let p = password().trim().to_string();
                                    if u.is_empty() || p.is_empty() {
                                        error.set(Some("Please fill in all fields.".into()));
                                        return;
                                    }
                                    loading.set(true);
                                    error.set(None);
                                    spawn(async move {
                                        match login(u, p).await {
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
                            if u.is_empty() || p.is_empty() {
                                error.set(Some("Please fill in all fields.".into()));
                                return;
                            }
                            loading.set(true);
                            error.set(None);
                            spawn(async move {
                                match login(u, p).await {
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
                            "Signing in..."
                        } else {
                            "Sign In"
                        }
                    }
                }

                p { class: "text-center text-sm text-gray-500 dark:text-gray-400 mt-4",
                    "Don't have an account? "
                    Link {
                        to: Route::Register {},
                        class: "text-blue-500 dark:text-blue-400 hover:text-blue-600 dark:hover:text-blue-300 font-medium",
                        "Sign up"
                    }
                }

                p { class: "text-center text-xs text-gray-400 dark:text-gray-500 mt-4",
                    "Default accounts: Alice, Bob, Charlie — password: password123"
                }
            }
        }
    }
}
