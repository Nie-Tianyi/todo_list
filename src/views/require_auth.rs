use crate::auth::AuthContext;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn RequireAuth() -> Element {
    let auth = use_context::<AuthContext>();
    let navigator = use_navigator();
    let authenticated = auth.is_authenticated();

    use_effect(move || {
        if !authenticated {
            navigator.push(Route::Login {});
        }
    });

    if !authenticated {
        return rsx! { div {} };
    }

    rsx! {
        Outlet::<Route> {}
    }
}
