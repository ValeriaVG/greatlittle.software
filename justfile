default: test

test:
    cargo test --workspace

build:
    cargo run

serve port="8000":
    python3 -m http.server {{port}} --directory dist

dev port="8000":
    cargo watch -w src -w Cargo.toml -i '*.html' -i '*.css' -i '*.js' -x 'run -- dev {{port}}'

dev-prod port="8000":
    cargo watch -w src -w Cargo.toml -i '*.html' -i '*.css' -i '*.js' -x 'run -- dev prod {{port}}'

preview port="8001":
    cargo watch -w src -w Cargo.toml -i '*.html' -i '*.css' -i '*.js' -x 'run --features preview -- preview {{port}}'

preview-build out=".preview-dist":
    cargo run --features preview -- preview build {{out}}

deploy: build
    command -v worker-build || cargo install worker-build
    npx wrangler deploy

write title description="":
    #!/usr/bin/env bash
    set -euo pipefail
    title={{ quote(title) }}
    description={{ quote(description) }}
    base=$(printf '%s' "$title" | tr '[:upper:]' '[:lower:]' \
        | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//')
    if [ -z "$base" ]; then
        echo "title produced empty slug" >&2
        exit 1
    fi
    slug="$base"
    if [ -e "content/blog/$slug" ]; then
        slug="$base-$(date +%Y-%m-%d)"
    fi
    if [ -e "content/blog/$slug" ]; then
        slug="$base-$(date +%Y-%m-%d-%H%M%S)"
    fi
    dir="content/blog/$slug"
    mkdir -p "$dir"
    file="$dir/index.md"
    sed -e "s|__TITLE__|$title|" \
        -e "s|__DESCRIPTION__|$description|" \
        -e "s|__DATE__|$(date -u +%Y-%m-%dT%H:%M:%SZ)|" \
        content/article.tpl.md > "$file"
    echo "$file"
