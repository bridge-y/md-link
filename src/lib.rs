use std::collections::HashMap;
use worker::*;

fn get_query_params(url: Url) -> Result<HashMap<String, String>> {
    Ok(url.query_pairs().into_owned().collect())
}

struct PageProperty {
    title: String,
    author: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let sec_val = env.secret("REQUEST_PATH")?.to_string();
    let req_path = format!("/{}", sec_val);

    let router = Router::new();

    router
        .get_async(&req_path, |mut req, ctx| async move {
            let url = req.url().unwrap();
            let params: HashMap<String, String> = url.query_pairs().into_owned().collect();
            let target_url = params.get("url").ok_or("error")?;
            Response::ok(format!("url={target_url}"))
        })
        .run(req, env)
        .await
}
