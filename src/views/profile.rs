use crate::auth::AuthContext;
use crate::backend::update_profile;
use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    let auth = use_context::<AuthContext>();
    let current_user = auth.user();

    let mut gender = use_signal(|| {
        current_user
            .as_ref()
            .and_then(|u| u.gender.clone())
            .unwrap_or_default()
    });
    let mut age = use_signal(|| {
        current_user
            .as_ref()
            .and_then(|u| u.age)
            .map(|a| a.to_string())
            .unwrap_or_default()
    });
    let mut job_title = use_signal(|| {
        current_user
            .as_ref()
            .and_then(|u| u.job_title.clone())
            .unwrap_or_default()
    });
    let mut email = use_signal(|| {
        current_user
            .as_ref()
            .and_then(|u| u.email.clone())
            .unwrap_or_default()
    });
    let mut error = use_signal(|| None::<String>);
    let mut success = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    if current_user.is_none() {
        return rsx! {
            div { class: "max-w-2xl mx-auto px-6 py-12",
                p { class: "text-gray-500", "Not logged in." }
            }
        };
    }

    let user = current_user.unwrap();

    rsx! {
        div { class: "max-w-2xl mx-auto px-6 py-12",
            h1 { class: "text-2xl font-bold text-gray-800 mb-6", "Profile" }

            if let Some(ref msg) = error() {
                div { class: "mb-4 p-3 bg-red-50 border border-red-100 text-red-600 text-sm rounded-lg",
                    "{msg}"
                }
            }
            if let Some(ref msg) = success() {
                div { class: "mb-4 p-3 bg-green-50 border border-green-100 text-green-600 text-sm rounded-lg",
                    "{msg}"
                }
            }

            div { class: "bg-white rounded-xl border border-gray-100 shadow-sm p-6 space-y-6",
                // Read-only fields
                div {
                    span { class: "text-sm text-gray-500", "Username" }
                    p { class: "text-lg font-medium text-gray-800", "{user.username}" }
                }
                div {
                    span { class: "text-sm text-gray-500", "User ID" }
                    p { class: "text-lg font-medium text-gray-800", "{user.id}" }
                }

                hr { class: "border-gray-100" }

                // Editable fields
                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-1.5",
                        r#for: "gender",
                        "Gender"
                    }
                    select {
                        id: "gender",
                        class: "w-full border border-gray-200 rounded-lg px-3 py-2.5 text-sm
                                focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                transition-shadow",
                        value: "{gender}",
                        oninput: move |e| {
                            gender.set(e.value());
                            error.set(None);
                        success.set(None);
                        },
                        option { value: "", "Prefer not to say" }
                        option { value: "Male", "Male" }
                        option { value: "Female", "Female" }
                        option { value: "Other", "Other" }
                    }
                }

                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-1.5",
                        r#for: "age",
                        "Age"
                    }
                    input {
                        id: "age",
                        r#type: "number",
                        class: "w-full border border-gray-200 rounded-lg px-3 py-2.5 text-sm
                                focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                transition-shadow",
                        placeholder: "Enter your age",
                        min: "0",
                        max: "150",
                        value: "{age}",
                        oninput: move |e| {
                            age.set(e.value());
                            error.set(None);
                        success.set(None);
                        },
                    }
                }

                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-1.5",
                        r#for: "job_title",
                        "Job Title"
                    }
                    input {
                        id: "job_title",
                        r#type: "text",
                        class: "w-full border border-gray-200 rounded-lg px-3 py-2.5 text-sm
                                focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                transition-shadow",
                        placeholder: "e.g. Software Engineer",
                        value: "{job_title}",
                        oninput: move |e| {
                            job_title.set(e.value());
                            error.set(None);
                        success.set(None);
                        },
                    }
                }

                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-1.5",
                        r#for: "email",
                        "Email"
                    }
                    input {
                        id: "email",
                        r#type: "email",
                        class: "w-full border border-gray-200 rounded-lg px-3 py-2.5 text-sm
                                focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                transition-shadow",
                        placeholder: "e.g. alice@example.com",
                        value: "{email}",
                        oninput: move |e| {
                            email.set(e.value());
                            error.set(None);
                        success.set(None);
                        },
                    }
                }

                button {
                    class: "w-full px-4 py-2.5 bg-blue-500 text-white text-sm font-medium rounded-lg
                            hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
                            disabled:opacity-50 disabled:cursor-not-allowed transition-colors",
                    disabled: loading(),
                    onclick: move |_| {
                        loading.set(true);
                        error.set(None);
                        success.set(None);

                        let g = if gender().trim().is_empty() {
                            None
                        } else {
                            Some(gender().trim().to_string())
                        };
                        let a = age().trim().parse::<i32>().ok();
                        let jt = if job_title().trim().is_empty() {
                            None
                        } else {
                            Some(job_title().trim().to_string())
                        };
                        let em = if email().trim().is_empty() {
                            None
                        } else {
                            Some(email().trim().to_string())
                        };

                        spawn(async move {
                            match update_profile(g, a, jt, em).await {
                                Ok(updated_user) => {
                                    auth.update_user(updated_user);
                                    success.set(Some("Profile updated successfully.".into()));
                                    loading.set(false);
                                }
                                Err(e) => {
                                    error.set(Some(e.to_string()));
                                    loading.set(false);
                                }
                            }
                        });
                    },
                    if loading() { "Saving..." } else { "Save Changes" }
                }
            }
        }
    }
}
