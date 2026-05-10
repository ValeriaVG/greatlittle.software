async function handleMarkdownNegotiation(context) {
  const accept = context.request.headers.get("Accept") || "";
  const wantsMarkdown = accept.split(",").some((v) => {
    const t = v.trim().toLowerCase();
    return t === "text/markdown" || t.startsWith("text/markdown;");
  });

  if (!wantsMarkdown) {
    return context.next();
  }

  const url = new URL(context.request.url);
  const path = url.pathname;

  let mdPath;
  if (path.endsWith("/")) {
    mdPath = path + "index.md";
  } else if (path.endsWith(".html")) {
    mdPath = path.replace(/\.html$/, ".md");
  } else {
    mdPath = path + ".md";
  }

  const mdUrl = new URL(mdPath, url.origin);
  const mdResponse = await context.env.ASSETS.fetch(new Request(mdUrl));
  if (mdResponse.ok) {
    const text = await mdResponse.text();
    const tokens = Math.ceil(text.length * 0.4);
    return new Response(text, {
      status: 200,
      headers: {
        "Content-Type": "text/markdown; charset=utf-8",
        "x-markdown-tokens": String(tokens),
        Vary: "Accept",
      },
    });
  }

  return context.next();
}

export const onRequest = [handleMarkdownNegotiation];