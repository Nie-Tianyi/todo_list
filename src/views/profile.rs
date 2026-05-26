use crate::auth::AuthContext;
use crate::backend::{change_password, update_profile};
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

    // Password change state
    let mut current_password = use_signal(String::new);
    let mut new_password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut pw_error = use_signal(|| None::<String>);
    let mut pw_success = use_signal(|| None::<String>);
    let mut pw_loading = use_signal(|| false);

    if current_user.is_none() {
        return rsx! {
            div { class: "max-w-2xl mx-auto px-6 py-12",
                p { class: "text-gray-500 dark:text-gray-400", "Not logged in." }
            }
        };
    }

    let user = current_user.unwrap();

    rsx! {
        div { class: "max-w-2xl mx-auto px-6 py-12",
            h1 { class: "text-2xl font-bold text-gray-800 dark:text-gray-100 mb-6",
                "Profile"
            }

            if let Some(ref msg) = error() {
                div { class: "mb-4 p-3 bg-red-50 dark:bg-red-900/30 border border-red-100 dark:border-red-800 text-red-600 dark:text-red-400 text-sm rounded-lg",
                    "{msg}"
                }
            }
            if let Some(ref msg) = success() {
                div { class: "mb-4 p-3 bg-green-50 dark:bg-green-900/30 border border-green-100 dark:border-green-800 text-green-600 dark:text-green-400 text-sm rounded-lg",
                    "{msg}"
                }
            }

            div { class: "bg-white dark:bg-gray-900 rounded-xl border border-gray-100 dark:border-gray-700 shadow-sm p-6 space-y-6",
                // Read-only fields
                div {
                    span { class: "text-sm text-gray-500 dark:text-gray-400", "Username" }
                    p { class: "text-lg font-medium text-gray-800 dark:text-gray-100",
                        "{user.username}"
                    }
                }
                div {
                    span { class: "text-sm text-gray-500 dark:text-gray-400", "User ID" }
                    p { class: "text-lg font-medium text-gray-800 dark:text-gray-100",
                        "{user.id}"
                    }
                }
                div {
                    span { class: "text-sm text-gray-500 dark:text-gray-400", "Role" }
                    p { class: "text-lg font-medium text-gray-800 dark:text-gray-100 capitalize",
                        "{user.role}"
                    }
                }

                hr { class: "border-gray-100 dark:border-gray-700" }

                // Editable fields
                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                        r#for: "gender",
                        "Gender"
                    }
                    select {
                        id: "gender",
                        class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2.5 text-sm dark:text-gray-100 dark:bg-gray-800
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
                        class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                        r#for: "age",
                        "Age"
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
                        oninput: move |e| {
                            age.set(e.value());
                            error.set(None);
                            success.set(None);
                        },
                    }
                }

                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                        r#for: "job_title",
                        "Job Title"
                    }
                    input {
                        id: "job_title",
                        r#type: "text",
                        class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2.5 text-sm dark:text-gray-100 dark:bg-gray-800
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
                        class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                        r#for: "email",
                        "Email"
                    }
                    input {
                        id: "email",
                        r#type: "email",
                        class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2.5 text-sm dark:text-gray-100 dark:bg-gray-800
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
                    if loading() {
                        "Saving..."
                    } else {
                        "Save Changes"
                    }
                }
            }

            // ── Change Password ──
            div { class: "bg-white dark:bg-gray-900 rounded-xl border border-gray-100 dark:border-gray-700 shadow-sm p-6 b-2 space-y-4",
                h2 { class: "text-lg font-semibold text-gray-800 dark:text-gray-100",
                    "Change Password"
                }

                if let Some(ref msg) = pw_error() {
                    div { class: "p-3 bg-red-50 dark:bg-red-900/30 border border-red-100 dark:border-red-800 text-red-600 dark:text-red-400 text-sm rounded-lg",
                        "{msg}"
                    }
                }
                if let Some(ref msg) = pw_success() {
                    div { class: "p-3 bg-green-50 dark:bg-green-900/30 border border-green-100 dark:border-green-800 text-green-600 dark:text-green-400 text-sm rounded-lg",
                        "{msg}"
                    }
                }

                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                        r#for: "current-password",
                        "Current Password"
                    }
                    input {
                        id: "current-password",
                        r#type: "password",
                        class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2.5 text-sm dark:text-gray-100 dark:bg-gray-800
                                focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                transition-shadow",
                        placeholder: "Enter current password",
                        value: "{current_password}",
                        oninput: move |e| {
                            current_password.set(e.value());
                            pw_error.set(None);
                            pw_success.set(None);
                        },
                    }
                }

                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                        r#for: "new-password",
                        "New Password"
                    }
                    input {
                        id: "new-password",
                        r#type: "password",
                        class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2.5 text-sm dark:text-gray-100 dark:bg-gray-800
                                focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                transition-shadow",
                        placeholder: "At least 6 characters",
                        value: "{new_password}",
                        oninput: move |e| {
                            new_password.set(e.value());
                            pw_error.set(None);
                            pw_success.set(None);
                        },
                    }
                }

                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1.5",
                        r#for: "confirm-password",
                        "Confirm New Password"
                    }
                    input {
                        id: "confirm-password",
                        r#type: "password",
                        class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2.5 text-sm dark:text-gray-100 dark:bg-gray-800
                                focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                                transition-shadow",
                        placeholder: "Re-enter new password",
                        value: "{confirm_password}",
                        oninput: move |e| {
                            confirm_password.set(e.value());
                            pw_error.set(None);
                            pw_success.set(None);
                        },
                        onkeydown: move |e| {
                            if e.key() == Key::Enter {
                                pw_loading.set(true);
                                pw_error.set(None);
                                pw_success.set(None);

                                let cp = current_password().trim().to_string();
                                let np = new_password().trim().to_string();
                                let cf = confirm_password().trim().to_string();

                                if cp.is_empty() || np.is_empty() || cf.is_empty() {
                                    pw_error.set(Some("All password fields are required.".into()));
                                    pw_loading.set(false);
                                    return;
                                }
                                if np.len() < 6 {
                                    pw_error.set(Some("New password must be at least 6 characters.".into()));
                                    pw_loading.set(false);
                                    return;
                                }
                                if np != cf {
                                    pw_error.set(Some("New passwords do not match.".into()));
                                    pw_loading.set(false);
                                    return;
                                }

                                spawn(async move {
                                    match change_password(cp, np).await {
                                        Ok(()) => {
                                            pw_success.set(Some("Password changed successfully.".into()));
                                            current_password.set(String::new());
                                            new_password.set(String::new());
                                            confirm_password.set(String::new());
                                            pw_loading.set(false);
                                        }
                                        Err(e) => {
                                            pw_error.set(Some(e.to_string()));
                                            pw_loading.set(false);
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
                    disabled: pw_loading(),
                    onclick: move |_| {
                        pw_loading.set(true);
                        pw_error.set(None);
                        pw_success.set(None);

                        let cp = current_password().trim().to_string();
                        let np = new_password().trim().to_string();
                        let cf = confirm_password().trim().to_string();

                        if cp.is_empty() || np.is_empty() || cf.is_empty() {
                            pw_error.set(Some("All password fields are required.".into()));
                            pw_loading.set(false);
                            return;
                        }
                        if np.len() < 6 {
                            pw_error.set(Some("New password must be at least 6 characters.".into()));
                            pw_loading.set(false);
                            return;
                        }
                        if np != cf {
                            pw_error.set(Some("New passwords do not match.".into()));
                            pw_loading.set(false);
                            return;
                        }

                        spawn(async move {
                            match change_password(cp, np).await {
                                Ok(()) => {
                                    pw_success.set(Some("Password changed successfully.".into()));
                                    current_password.set(String::new());
                                    new_password.set(String::new());
                                    confirm_password.set(String::new());
                                    pw_loading.set(false);
                                }
                                Err(e) => {
                                    pw_error.set(Some(e.to_string()));
                                    pw_loading.set(false);
                                }
                            }
                        });
                    },
                    if pw_loading() {
                        "Changing..."
                    } else {
                        "Change Password"
                    }
                }
            }
        }
    }
}
