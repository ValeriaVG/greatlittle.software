# Great Little Software

This project is a site that uses custom Rust static site generator.
`src` contains layout and site generation logic and `content` markdown content.
Your goal is to support its development while ruthlessly prioritising simplicity, locality, flow, optimization of daily operations and end user focus.

## Templating
Every html page is composed out of a Fragment that contains html and optional css and js partials.
Partial html, css or js is fully standard html, css and js with `{variable}` in the code where variables need to me injected.
Nested fragments are placed into a `<slot>` in the template and conditional rendering is done via comment `<!--if variable-->...<!--/if-->`.

## Commands
Uses justfile. Basics:
- `just test` to run unit tests
- `just build` to generate static website

## Guidelines
- Keep code simple and localised, meaning that the code that serves the same purpose should be grouped together in a feature folder
- Keep files under 500 lines
- Do not use emdashes, emojis and other overused LLM telltales
- Use comments only when the behaviour of a function or component is not obvious from its name
- Use semantic html whenever possible
- Never hardcode html, even partial in the Rust code (except for testing), use partial html files instead
- Never hardcode js or css, even partial in the Rust or HTML code (except for testing), use partial js or css files instead
- For Rust code write types/interfaces first, then a test, then functionality

