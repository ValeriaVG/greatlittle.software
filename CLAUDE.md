# Great Little Software

Uses Rust to generate static website from modular html via proc_macros `block!` and `html_template!`.

## Structure
- ./content is intended for markdown articles content and assets used on the website
- ./src contains the generation logic and site specific styling and modules
- ./macros contain custom macros

## Commands
Project uses justfile, check ./justfile for all commands, the basic ones are:
- just test
- just build

## Guidelines
- Keep files under 500 lines
- Group code by domain scope where possible: e.g. `src/home` contains everything one need to build home page and `src/html` contains shared logic for html page generation



