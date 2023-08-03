use html_escape::decode_html_entities;
use reqwest;
use std::collections::HashMap;
use tl;
use worker::*;

// extract query parameters from the URL
fn get_query_params(url: Url) -> Result<HashMap<String, String>> {
    Ok(url.query_pairs().into_owned().collect())
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Fetch the secret value for the REQUEST_PATH environment variable
    // to escape access via bots
    let sec_val = env.secret("REQUEST_PATH")?.to_string();
    let req_path = format!("/{}", sec_val);

    let router = Router::new();

    router
        .get_async(&req_path, |req, _ctx| async move {
            let url = req.url().unwrap();
            let params = get_query_params(url)?;

            // Extract the "url" query parameter and get the web page title
            let target_url = params.get("url").ok_or("error")?;
            let title = match get_page_title(target_url).await {
                Some(s) => s,
                None => return Response::error("fail to get page title", 500),
            };

            // Create and return a markdown link using the title and the target URL
            Response::ok(make_markdown_link(&title, &target_url))
        })
        .run(req, env)
        .await
}

// get title of web page
async fn get_page_title(url: &str) -> Option<String> {
    let response = reqwest::get(url).await;
    let body = match response {
        Ok(val) => val.text().await.expect("fail to get body"),
        Err(_) => return None,
    };

    let dom = tl::parse(&body, tl::ParserOptions::default()).expect("parse error");
    let parser = dom.parser();
    let handler = dom
        .query_selector("title")
        .and_then(|mut iter| iter.next())
        .unwrap();
    let node = handler.get(parser).unwrap();
    console_log!("{}", format!("{:?}", node.inner_text(parser)));
    let title = node.inner_text(parser).to_string();
    Some(decode_html_entities(&title).to_string())
}


// create a markdown link
fn make_markdown_link(title: &str, url: &str) -> String {
    format!("[{title}]({url})")
}
