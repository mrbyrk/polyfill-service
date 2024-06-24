use polyfill_library::{
    buffer::Buffer, get_polyfill_string::get_polyfill_string_stream,
    polyfill_parameters::get_polyfill_parameters, Env,
};
use std::sync::Arc;

fn parse_library_version(version: &str) -> Option<String> {
    return match version {
        "3.111.0" => Some("3.111.0".to_owned()),
        "4.8.0" => Some("4.8.0".to_owned()),
        _ => None,
    };
}

pub(crate) async fn polyfill(
    request: &worker::Request,
    env: Arc<Env>,
) -> worker::Result<worker::Response> {
    let parameters = get_polyfill_parameters(request);

    let _library = match parse_library_version(&parameters.version) {
        Some(library) => library,
        None => {
            let mut headers = worker::Headers::new();
            headers.set("Cache-Control", "public, s-maxage=31536000, max-age=604800, stale-while-revalidate=604800, stale-if-error=604800, immutable")?;

            return worker::Response::error("requested version does not exist", 400);
        }
    };
    let version = parameters.version.clone();
    let mut headers = worker::Headers::new();
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Methods", "GET,HEAD,OPTIONS")?;
    headers.set("X-Compress-Hint", "on")?;
    headers.set("Content-Type", "text/javascript; charset=UTF-8")?;
    headers.set("Cache-Control", "public, s-maxage=31536000, max-age=604800, stale-while-revalidate=604800, stale-if-error=604800, immutable")?;
    // We need "Vary: User-Agent" in the browser cache because a browser
    // may update itself to a version which needs different polyfills
    // So we need to have it ignore the browser cached bundle when the user-agent changes.
    headers.set("Vary", "User-Agent, Accept-Encoding")?;
    let mut res_body = Buffer::new();

    get_polyfill_string_stream(&mut res_body, &parameters, env, &version)
        .await
        .map_err(|err| {
            worker::Error::RustError(format!("failed to get_polyfill_string_stream: {err}"))
        })?;

    Ok(worker::Response::ok(res_body.into_str())?.with_headers(headers))
}
