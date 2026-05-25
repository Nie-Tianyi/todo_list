You are an expert [0.7 Dioxus](https://dioxuslabs.com/learn/0.7) assistant. Dioxus 0.7 changes every api in dioxus. Only use this up to date documentation. `cx`, `Scope`, and `use_state` are gone

Provide concise code examples with detailed descriptions

# Dioxus Dependency

You can add Dioxus to your `Cargo.toml` like this:

```toml
[dependencies]
dioxus = { version = "0.7.1" }

[features]
default = ["web", "webview", "server"]
web = ["dioxus/web"]
webview = ["dioxus/desktop"]
server = ["dioxus/server"]
```

# Launching your application

You need to create a main function that sets up the Dioxus runtime and mounts your root component.

```rust
use dioxus::prelude::*;

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	rsx! { "Hello, Dioxus!" }
}
```

Then serve with `dx serve`:

```sh
curl -sSL http://dioxus.dev/install.sh | sh
dx serve
```

# UI with RSX

```rust
rsx! {
	div {
		class: "container", // Attribute
		color: "red", // Inline styles
		width: if condition { "100%" }, // Conditional attributes
		"Hello, Dioxus!"
	}
	// Prefer loops over iterators
	for i in 0..5 {
		div { "{i}" } // use elements or components directly in loops
	}
	if condition {
		div { "Condition is true!" } // use elements or components directly in conditionals
	}

	{children} // Expressions are wrapped in brace
	{(0..5).map(|i| rsx! { span { "Item {i}" } })} // Iterators must be wrapped in braces
}
```

# Assets

The asset macro can be used to link to local files to use in your project. All links start with `/` and are relative to the root of your project.

```rust
rsx! {
	img {
		src: asset!("/assets/image.png"),
		alt: "An image",
	}
}
```

## Styles

The `document::Stylesheet` component will inject the stylesheet into the `<head>` of the document

```rust
rsx! {
	document::Stylesheet {
		href: asset!("/assets/styles.css"),
	}
}
```

# Components

Components are the building blocks of apps

* Component are functions annotated with the `#[component]` macro.
* The function name must start with a capital letter or contain an underscore.
* A component re-renders only under two conditions:
	1.  Its props change (as determined by `PartialEq`).
	2.  An internal reactive state it depends on is updated.

```rust
#[component]
fn Input(mut value: Signal<String>) -> Element {
	rsx! {
		input {
            value,
			oninput: move |e| {
				*value.write() = e.value();
			},
			onkeydown: move |e| {
				if e.key() == Key::Enter {
					value.write().clear();
				}
			},
		}
	}
}
```

Each component accepts function arguments (props)

* Props must be owned values, not references. Use `String` and `Vec<T>` instead of `&str` or `&[T]`.
* Props must implement `PartialEq` and `Clone`.
* To make props reactive and copy, you can wrap the type in `ReadOnlySignal`. Any reactive state like memos and resources that read `ReadOnlySignal` props will automatically re-run when the prop changes.

# State

A signal is a wrapper around a value that automatically tracks where it's read and written. Changing a signal's value causes code that relies on the signal to rerun.

## Local State

The `use_signal` hook creates state that is local to a single component. You can call the signal like a function (e.g. `my_signal()`) to clone the value, or use `.read()` to get a reference. `.write()` gets a mutable reference to the value.

Use `use_memo` to create a memoized value that recalculates when its dependencies change. Memos are useful for expensive calculations that you don't want to repeat unnecessarily.

```rust
#[component]
fn Counter() -> Element {
	let mut count = use_signal(|| 0);
	let mut doubled = use_memo(move || count() * 2); // doubled will re-run when count changes because it reads the signal

	rsx! {
		h1 { "Count: {count}" } // Counter will re-render when count changes because it reads the signal
		h2 { "Doubled: {doubled}" }
		button {
			onclick: move |_| *count.write() += 1, // Writing to the signal rerenders Counter
			"Increment"
		}
		button {
			onclick: move |_| count.with_mut(|count| *count += 1), // use with_mut to mutate the signal
			"Increment with with_mut"
		}
	}
}
```

## Context API

The Context API allows you to share state down the component tree. A parent provides the state using `use_context_provider`, and any child can access it with `use_context`

```rust
#[component]
fn App() -> Element {
	let mut theme = use_signal(|| "light".to_string());
	use_context_provider(|| theme); // Provide a type to children
	rsx! { Child {} }
}

#[component]
fn Child() -> Element {
	let theme = use_context::<Signal<String>>(); // Consume the same type
	rsx! {
		div {
			"Current theme: {theme}"
		}
	}
}
```

# Async

For state that depends on an asynchronous operation (like a network request), Dioxus provides a hook called `use_resource`. This hook manages the lifecycle of the async task and provides the result to your component.

* The `use_resource` hook takes an `async` closure. It re-runs this closure whenever any signals it depends on (reads) are updated
* The `Resource` object returned can be in several states when read:
1. `None` if the resource is still loading
2. `Some(value)` if the resource has successfully loaded

```rust
let mut dog = use_resource(move || async move {
	// api request
});

match dog() {
	Some(dog_info) => rsx! { Dog { dog_info } },
	None => rsx! { "Loading..." },
}
```

# Routing

All possible routes are defined in a single Rust `enum` that derives `Routable`. Each variant represents a route and is annotated with `#[route("/path")]`. Dynamic Segments can capture parts of the URL path as parameters by using `:name` in the route string. These become fields in the enum variant.

The `Router<Route> {}` component is the entry point that manages rendering the correct component for the current URL.

You can use the `#[layout(NavBar)]` to create a layout shared between pages and place an `Outlet<Route> {}` inside your layout component. The child routes will be rendered in the outlet.

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
	#[layout(NavBar)] // This will use NavBar as the layout for all routes
		#[route("/")]
		Home {},
		#[route("/blog/:id")] // Dynamic segment
		BlogPost { id: i32 },
}

#[component]
fn NavBar() -> Element {
	rsx! {
		a { href: "/", "Home" }
		Outlet<Route> {} // Renders Home or BlogPost
	}
}

#[component]
fn App() -> Element {
	rsx! { Router::<Route> {} }
}
```

```toml
dioxus = { version = "0.7.1", features = ["router"] }
```

# Fullstack

Fullstack enables server rendering and ipc calls. It uses Cargo features (`server` and a client feature like `web`) to split the code into a server and client binaries.

```toml
dioxus = { version = "0.7.1", features = ["fullstack"] }
```

## Server Functions

Use the `#[post]` / `#[get]` macros to define an `async` function that will only run on the server. On the server, this macro generates an API endpoint. On the client, it generates a function that makes an HTTP request to that endpoint.

```rust
#[post("/api/double/:path/&query")]
async fn double_server(number: i32, path: String, query: i32) -> Result<i32, ServerFnError> {
	tokio::time::sleep(std::time::Duration::from_secs(1)).await;
	Ok(number * 2)
}
```

## Hydration

Hydration is the process of making a server-rendered HTML page interactive on the client. The server sends the initial HTML, and then the client-side runs, attaches event listeners, and takes control of future rendering.

### Errors
The initial UI rendered by the component on the client must be identical to the UI rendered on the server.

* Use the `use_server_future` hook instead of `use_resource`. It runs the future on the server, serializes the result, and sends it to the client, ensuring the client has the data immediately for its first render.
* Any code that relies on browser-specific APIs (like accessing `localStorage`) must be run *after* hydration. Place this code inside a `use_effect` hook.

---

# Project: Todo List

## Architecture

Fullstack Dioxus 0.7 app with SQLite storage, JWT authentication, and a Kanban board UI styled with Tailwind CSS.

- **Client:** WASM (web feature), renders Kanban board, handles user interactions
- **Server:** Axum (server feature), serves API endpoints and static assets
- **Database:** SQLite via `sqlx 0.8`, file `todo_list.db` at project root
- **Auth:** Argon2 password hashing + JWT bearer tokens

## Dependencies

```toml
dioxus = { version = "0.7.1", features = ["router", "fullstack"] }
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio"], optional = true }
argon2 = "0.5"
jsonwebtoken = "9"
axum = { version = "0.8", optional = true }
rand = "0.8"
http = "1"
```

## File Structure

```
src/
├── main.rs          # Route enum, App component, AuthContext provider
├── models.rs        # Task, TaskStatus, Priority, User, LoginResponse
├── backend.rs       # Server functions (CRUD + auth), SQLite migrations
├── auth.rs          # AuthSession extractor, AuthContext, JWT logic
├── views/
│   ├── mod.rs
│   ├── navbar.rs        # Layout: nav bar + Outlet, reads AuthContext
│   ├── todos.rs         # Main Kanban board page (protected)
│   ├── profile.rs       # User profile page (protected)
│   ├── login.rs         # Login form
│   ├── register.rs      # Registration form
│   ├── require_auth.rs  # Route guard layout
│   └── page_not_found.rs
└── components/
    ├── mod.rs
    ├── kanban_column.rs # Single status column
    ├── task_card.rs     # Task card with claim/move actions
    └── task_form.rs     # New-task creation form
```

## Routes

```rust
#[layout(Navbar)]
    #[route("/login")]     Login {}          // public
    #[route("/register")]  Register {}       // public
    #[layout(RequireAuth)]
        #[route("/")]      Todos {}          // protected
        #[route("/profile")] Profile {}      // protected
#[route("/:..segments")]    PageNoteFound {} // 404
```

## Authentication

### Server-side

- Passwords hashed with **Argon2id** (random salt per password)
- **JWT** tokens (HS256), 24h expiry, stored in `Authorization: Bearer <token>` header
- `AuthSession` struct implements `FromRequestParts<()>` — reads and validates JWT from request headers
- Server functions declare the extractor in the attribute: `#[get("/api/tasks", auth: crate::auth::AuthSession)]`
- The `auth` variable is injected by the macro and available in the function body

### Client-side

- `AuthContext(Signal<Option<AuthState>>)` — provided via `use_context_provider` in `App`
- `AuthState { user: User, token: String }` — stores user info + JWT
- `auth.login(state)` — calls `dioxus::fullstack::set_request_headers()` to attach bearer token to all subsequent server function requests
- `auth.logout()` — calls `dioxus::fullstack::clear_request_headers()`
- `RequireAuth` layout component — reads `AuthContext`, redirects to `/login` via `use_effect` + `navigator.push()` if not authenticated

### Server functions

| Endpoint | Auth | Purpose |
|----------|------|---------|
| `POST /api/login` | none | Validate credentials, return JWT |
| `POST /api/register` | none | Create user (checks duplicate), return JWT |
| `GET /api/tasks` | AuthSession | List all tasks |
| `POST /api/tasks` | AuthSession | Create a task |
| `POST /api/tasks/save` | AuthSession | Upsert a task |

## Key Patterns

- **No `cx` / `Scope` / `use_state`** — Dioxus 0.7 uses `use_signal`, `use_resource`, `use_effect`, `use_context`
- **Prop drilling** of `Signal<T>` to child components (not context for everything)
- **Event handler closures** cannot be shared across different event types — inline the logic or use `spawn`
- **Server-only code** gated with `#[cfg(feature = "server")]` in `backend.rs` and `auth.rs`
- **Server-only extractors** declared in attribute macros (not function signatures): `#[post("/path", extractor: Type)]`
- **Database** auto-migrates via `run_migrations()` on first pool creation; seeds default tasks + 3 users (Alice/Bob/Charlie, password: `password123`)
