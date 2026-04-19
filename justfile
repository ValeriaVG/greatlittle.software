default: test

test:
    cargo test --workspace

build:
    cargo run

serve port="8000":
    python3 -m http.server {{port}} --directory dist
