#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(clippy::missing_docs_in_private_items)]
mod pages;
mod polyfill;

use crate::polyfill::polyfill;
use pages::home;
use std::collections::HashMap;
use std::str;
use std::sync::Arc;

pub use polyfill_library::Env;

const APPLICATION_JSON: &str = "application/json";

macro_rules! library_json {
    ( $file:expr ) => {{
        let mut headers = worker::Headers::new();
        headers.set("content-type", APPLICATION_JSON)?;
        headers.set("x-compress-hint", "on")?;
        headers.set("surrogate-key", "website")?;
        headers.set(
            "Cache-Control",
            "max-age=86400, stale-while-revalidate=86400, stale-if-error=86400",
        )?;

        return Ok(worker::Response::ok(include_str!($file))?.with_headers(headers));
    }};
}

pub async fn handle_request(
    req: worker::Request,
    env: Arc<Env>,
) -> worker::Result<worker::Response> {
    match req.method() {
        worker::Method::Options => {
            let mut headers = worker::Headers::new();
            headers.set("allow", "OPTIONS, GET, HEAD")?;
            headers.set("Cache-Control", "public, s-maxage=31536000, max-age=604800, stale-while-revalidate=604800, stale-if-error=604800, immutable")?;

            return Ok(worker::Response::ok("")?
                .with_status(200)
                .with_headers(headers));
        }
        worker::Method::Connect
        | worker::Method::Delete
        | worker::Method::Patch
        | worker::Method::Post
        | worker::Method::Put
        | worker::Method::Trace => {
            let mut headers = worker::Headers::new();
            headers.set("allow", "GET, HEAD")?;

            return Ok(
                worker::Response::error("This method is not allowed\n", 405)?.with_headers(headers),
            );
        }
        _ => {}
    };
    let path = req.path();
    match path.as_str() {
        "/" => {
            let mut headers = worker::Headers::new();

            headers.set("content-type", "text/html; charset=utf-8")?;
            headers.set("x-compress-hint", "on")?;
            // Enables the cross-site scripting filter built into most modern web browsers.
            headers.set("X-XSS-Protection", "1; mode=block")?;
            // Prevents MIME-sniffing a response away from the declared content type.
            headers.set("X-Content-Type-Options", "nosniff")?;
            // The Referrer-Policy header governs which referrer information, sent in the Referer header, should be included with requests made.
            // Send a full URL when performing a same-origin request, but only send the origin of the document for other cases.
            headers.set("Referrer-Policy", "origin-when-cross-origin")?;
            // Ensure the site is only served over HTTPS and reduce the chances of someone performing a MITM attack.
            headers.set(
                "Strict-Transport-Security",
                "max-age=31536000; includeSubdomains; preload",
            )?;
            headers.set(
                "Cache-Control",
                "max-age=60, stale-while-revalidate=60, stale-if-error=86400",
            )?;

            return Ok(worker::Response::ok(home())?.with_headers(headers));
        }
        "/img/logo.svg" => {
            let mut headers = worker::Headers::new();
            headers.set("content-type", "image/svg+xml")?;
            headers.set("x-compress-hint", "on")?;
            headers.set("surrogate-key", "website")?;

            return Ok(worker::Response::ok(include_str!("logo.svg"))?.with_headers(headers));
        }
        "/robots.txt" => {
            return worker::Response::ok("User-agent: *\nDisallow:");
        }

        "/v3/json/library-3.101.0.json" => {
            library_json!("json/library-3.101.0.json")
        }
        "/v3/json/library-3.103.0.json" => {
            library_json!("json/library-3.103.0.json")
        }
        "/v3/json/library-3.104.0.json" => {
            library_json!("json/library-3.104.0.json")
        }
        "/v3/json/library-3.108.0.json" => {
            library_json!("json/library-3.108.0.json")
        }
        "/v3/json/library-3.109.0.json" => {
            library_json!("json/library-3.109.0.json")
        }
        "/v3/json/library-3.110.1.json" => {
            library_json!("json/library-3.110.1.json")
        }
        "/v3/json/library-3.111.0.json" => {
            library_json!("json/library-3.111.0.json")
        }
        "/v3/json/library-3.27.4.json" => {
            library_json!("json/library-3.27.4.json")
        }
        "/v3/json/library-3.34.0.json" => {
            library_json!("json/library-3.34.0.json")
        }
        "/v3/json/library-3.39.0.json" => {
            library_json!("json/library-3.39.0.json")
        }
        "/v3/json/library-3.40.0.json" => {
            library_json!("json/library-3.40.0.json")
        }
        "/v3/json/library-3.41.0.json" => {
            library_json!("json/library-3.41.0.json")
        }
        "/v3/json/library-3.42.0.json" => {
            library_json!("json/library-3.42.0.json")
        }
        "/v3/json/library-3.46.0.json" => {
            library_json!("json/library-3.46.0.json")
        }
        "/v3/json/library-3.48.0.json" => {
            library_json!("json/library-3.48.0.json")
        }
        "/v3/json/library-3.50.2.json" => {
            library_json!("json/library-3.50.2.json")
        }
        "/v3/json/library-3.51.0.json" => {
            library_json!("json/library-3.51.0.json")
        }
        "/v3/json/library-3.52.0.json" => {
            library_json!("json/library-3.52.0.json")
        }
        "/v3/json/library-3.52.1.json" => {
            library_json!("json/library-3.52.1.json")
        }
        "/v3/json/library-3.52.2.json" => {
            library_json!("json/library-3.52.2.json")
        }
        "/v3/json/library-3.52.3.json" => {
            library_json!("json/library-3.52.3.json")
        }
        "/v3/json/library-3.53.1.json" => {
            library_json!("json/library-3.53.1.json")
        }
        "/v3/json/library-3.89.4.json" => {
            library_json!("json/library-3.89.4.json")
        }
        "/v3/json/library-3.96.0.json" => {
            library_json!("json/library-3.96.0.json")
        }
        "/v3/json/library-3.98.0.json" => {
            library_json!("json/library-3.98.0.json")
        }

        // FIXME: should be v4
        "/v3/json/library-4.8.0.json" => {
            library_json!("json/library-4.8.0.json")
        }

        _ => {
            if path == "/v2/polyfill.js" || path == "/v2/polyfill.min.js" {
                let mut url = req
                    .url()
                    .map_err(|err| worker::Error::RustError(format!("failed to get URL: {err}")))?;

                url.set_path(&(String::from("/v3") + &path[3..]));
                url.query_pairs_mut().append_pair("version", "3.25.1");

                let search_params = url
                    .query_pairs()
                    .into_owned()
                    .collect::<HashMap<String, String>>();

                if !search_params.contains_key("unknown") {
                    url.query_pairs_mut().append_pair("unknown", "ignore");
                }

                let req_init = worker::RequestInit {
                    body: None,
                    headers: req.headers().clone(),
                    cf: worker::CfProperties::default(),
                    method: req.method(),
                    redirect: worker::RequestRedirect::Follow,
                };
                let req2 =
                    worker::Request::new_with_init(&url.to_string(), &req_init).map_err(|err| {
                        worker::Error::RustError(format!("failed to build new request: {err}"))
                    })?;

                polyfill(&req2, env).await
            }
            // FIXME: add v4
            else if path == "/v3/polyfill.min.js" || path == "/v3/polyfill.js" {
                polyfill(&req, env).await
            } else {
                let mut headers = worker::Headers::new();
                headers.set("Cache-Control", "public, s-maxage=31536000, max-age=604800, stale-while-revalidate=604800, stale-if-error=604800, immutable")?;

                return Ok(worker::Response::error(format!("{path}: Not Found"), 404)?
                    .with_headers(headers));
            }
        }
    }
}
