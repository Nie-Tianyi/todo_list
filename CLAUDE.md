# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development

```bash
# Run the web app (also auto-compiles Tailwind CSS in Dioxus 0.7+)
dx serve --platform web

# Run as desktop app
dx serve --platform desktop

# Manually compile Tailwind CSS (for custom plugins or when auto-tailwind isn't used)
npm run build:css       # one-shot
npm run watch:css       # watch mode
```

## Architecture

This is a **Dioxus 0.7 fullstack web app** with the `router` feature. Routes are defined as a Rust enum in `src/main.rs`, with `#[layout]` and `#[route]` attributes — the router maps URL patterns to components directly.

**Route structure** (`src/main.rs`):
- `Navbar` is a **layout** (applied to all child routes). It renders a nav bar and an `<Outlet>` where the active route's content appears.
- `Route::Todos` — `/`
- `Route::Profile { id: i32 }` — `/blog/:id`

**Module layout**:
- `src/main.rs` — entry point: defines `Route` enum, `App` component, launches the app
- `src/views/` — route-level components (Navbar layout, Todos page, Profile page)
- `src/components/` — shared/reusable UI components (currently empty scaffold)
- `assets/` — static assets served to the browser (`tailwind.css` is the compiled output)
- `tailwind.css` (repo root) — Tailwind CSS v4 input file (`@import "tailwindcss"`), used as compilation source

## Tailwind CSS

Dioxus 0.7+ has **built-in Tailwind support**: `dx serve` detects `tailwind.css` in the project root and auto-compiles it to `assets/tailwind.css`. The README notes this works without manual setup.

If custom Tailwind plugins are needed, the CLI approach applies:
- Input: `tailwind.css` (root, `@import "tailwindcss"`)
- Output: `assets/tailwind.css` (loaded via `asset!("/assets/tailwind.css")` in `App`)
- The compiled output is in `.gitignore`; the source is tracked

## Routing Pattern

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]        // wraps all child routes with this component
        #[route("/")]        // URL pattern
        Todos {},            // renders `views::Todos` component
        #[route("/blog/:id")]
        Profile { id: i32 }, // dynamic param passed as component prop
}
```

- Variant name maps to a component of the same name (e.g. `Todos {}` → `Todos` component)
- Dynamic params (`:id`) implement `FromStr` and `Display`, passed as component props
- The `#[layout]` component must render `<Outlet::<Route> {}>` where child route content goes
