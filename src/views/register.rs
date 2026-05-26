use crate::auth::AuthContext;
use crate::backend::register;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Register() -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let mut gender = use_signal(String::new);
    let mut age = use_signal(String::new);
    let mut job_title = use_signal(String::new);
    let mut email = use_signal(String::new);
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
                    h2 { class: "text-2xl font-bold text-gray-800 dark:text-gray-100 mb-2", "Create Account" }
                    p { class: "text-sm text-gray-500 dark:text-gray-400 mb-6", "Sign up to start managing your tasks." }

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
                            class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                            r#for: "confirm",
                            "Confirm Password"
                        }
                        input {
                            id: "confirm",
                            r#type: "password",
                            class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2.5 text-sm dark:text-gray-100 dark:bg-gray-800
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
                                    let g = if gender().trim().is_empty() { None } else { Some(gender().trim().to_string()) };
                                    let a = age().trim().parse::<i32>().ok();
                                    let jt = if job_title().trim().is_empty() { None } else { Some(job_title().trim().to_string()) };
                                    let em = if email().trim().is_empty() { None } else { Some(email().trim().to_string()) };
                                    spawn(async move {
                                        match register(u, p, g, a, jt, em).await {
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

                    // ── Optional profile fields ─────────────────
                    div { class: "mb-4",
                        label {
                            class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                            r#for: "gender",
                            "Gender (optional)"
                        }
                        select {
                            id: "gender",
                            class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2.5 text-sm dark:text-gray-100 dark:bg-gray-800
                                    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                    transition-shadow",
                            value: "{gender}",
                            oninput: move |e| { gender.set(e.value()); },
                            option { value: "", "Prefer not to say" }
                            option { value: "Male", "Male" }
                            option { value: "Female", "Female" }
                            option { value: "Other", "Other" }
                        }
                    }

                    div { class: "mb-4",
                        label {
                            class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                            r#for: "age",
                            "Age (optional)"
                        }
                        input {
                            id: "age",
                            r#type: "number",
                            class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2.5 text-sm dark:text-gray-100 dark:bg-gray-800
                                    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                    transition-shadow",
                            placeholder: "Enter your age",
                            min: "0",
                            max: "150",
                            value: "{age}",
                            oninput: move |e| { age.set(e.value()); },
                        }
                    }

                    div { class: "mb-4",
                        label {
                            class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                            r#for: "job_title",
                            "Job Title (optional)"
                        }
                        input {
                            id: "job_title",
                            r#type: "text",
                            class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2.5 text-sm dark:text-gray-100 dark:bg-gray-800
                                    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                    transition-shadow",
                            placeholder: "e.g. Software Engineer",
                            value: "{job_title}",
                            oninput: move |e| { job_title.set(e.value()); },
                        }
                    }

                    div { class: "mb-6",
                        label {
                            class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                            r#for: "email",
                            "Email (optional)"
                        }
                        input {
                            id: "email",
                            r#type: "email",
                            class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2.5 text-sm dark:text-gray-100 dark:bg-gray-800
                                    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                    transition-shadow",
                            placeholder: "e.g. alice@example.com",
                            value: "{email}",
                            oninput: move |e| { email.set(e.value()); },
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
                            let g = if gender().trim().is_empty() { None } else { Some(gender().trim().to_string()) };
                            let a = age().trim().parse::<i32>().ok();
                            let jt = if job_title().trim().is_empty() { None } else { Some(job_title().trim().to_string()) };
                            let em = if email().trim().is_empty() { None } else { Some(email().trim().to_string()) };
                            spawn(async move {
                                match register(u, p, g, a, jt, em).await {
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

                    p { class: "text-center text-sm text-gray-500 dark:text-gray-400 mt-4",
                        "Already have an account? "
                        Link {
                            to: Route::Login {},
                            class: "text-blue-500 dark:text-blue-400 hover:text-blue-600 dark:hover:text-blue-300 font-medium",
                            "Sign in"
                        }
                    }
                }
            }
        }
    }
}
