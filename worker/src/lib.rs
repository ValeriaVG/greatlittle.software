use worker::{event, Env, Fetcher, Method, Request, Response, Result};

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let accept = req
        .headers()
        .get("Accept")
        .unwrap_or_default()
        .unwrap_or_default();

    if wants_markdown(&accept) {
        return handle_markdown(req, env).await;
    }

    let assets: Fetcher = env.assets("ASSETS")?;
    assets.fetch_request(req).await
}

fn wants_markdown(accept: &str) -> bool {
    accept.split(',').any(|v| {
        let t = v.trim().to_lowercase();
        t == "text/markdown" || t.starts_with("text/markdown;")
    })
}

async fn handle_markdown(req: Request, env: Env) -> Result<Response> {
    let url = req.url()?;
    let path = url.path();

    let md_path = if path.ends_with('/') {
        format!("{}index.md", path)
    } else if path.ends_with(".html") {
        path.replace(".html", ".md")
    } else {
        format!("{}.md", path)
    };

    let md_url = format!("https://assets.local{}", md_path);
    let md_req = Request::new(&md_url, Method::Get)?;
    let assets: Fetcher = env.assets("ASSETS")?;

    match assets.fetch_request(md_req).await {
        Ok(resp) if resp.status_code() == 200 => {
            let headers = worker::Headers::new();
            headers.set("Content-Type", "text/markdown; charset=utf-8")?;
            headers.set("Vary", "Accept")?;
            Ok(resp.with_headers(headers))
        }
        _ => {
            let assets: Fetcher = env.assets("ASSETS")?;
            assets.fetch_request(req).await
        }
    }
}