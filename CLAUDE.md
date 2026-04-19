# Great Little Software

Rust static site generator.
Components are Rust functions that return `Fragment { html, css, js }`.
Composition is plain function calls; child components plug into parents via `<slot>` elements in the HTML template.

## Authoring a component

A component is a `.html` file with optional sibling `.css` and `.js`, registered via the `html_template!` proc macro:

```rust
use macros::html_template;
use crate::html::Fragment;

html_template!(intro,  "src/home/intro");
html_template!(layout, "src/home/layout");
```

Each call generates `fn <name>(...) -> Fragment`. Signature comes from the template:
- `{title}` → `title: &str` scalar arg
- `{image.src}` → `image: &RenderIntroImage` struct arg (fields become `String`)
- `<slot name="children"></slot>` → `children: Fragment` arg (its `css`/`js` bubble up into the returned fragment)

Repeated placeholders/slots with the same name share one argument. Same name cannot be used as two different kinds.

## Page assembly

`src/main.rs` calls the root component and passes the returned `Fragment` to `html::finalize`, which inlines bubbled-up CSS as `<style>` before `</head>` and JS as `<script>` before `</body>`. No other stitching layer exists.

CSS has no automatic scoping — authors write plain selectors and namespace by hand if needed.

## Structure
- `./content` — markdown articles and assets (not yet wired into generation)
- `./src` — generation entry point, shared runtime (`src/html`), and per-page modules (e.g. `src/home`)
- `./macros` — proc-macro crate providing `html_template!`

## Commands
Uses justfile. Basics:
- `just test`
- `just build`

## Guidelines
- Keep files under 500 lines
- Group code by domain scope: e.g. `src/home` holds everything needed to build the home page; `src/html` holds shared page-generation logic
- Prefer extending `html_template!` over introducing a second templating path
- Do not use emdashes, emojis and other overused LLM telltales
- Be precise and consice
