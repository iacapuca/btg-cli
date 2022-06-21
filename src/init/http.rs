use crate::init::display_error_info;
use anyhow::Result;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use oauth2::url;
use tokio::sync::mpsc;

async fn handle_callback(req: Request<Body>, tx: mpsc::Sender<String>) -> Result<Response<Body>> {
    match req.uri().path() {
        "/" => {
            let params = match req
                .uri()
                .query()
                .map(|v| url::form_urlencoded::parse(v.as_bytes()))
            {
                Some(values) => values,
                None => anyhow::bail!("Failed to map params from HTTP response"),
            };

            // Get authorization code and csrf state
            let mut params_values: Vec<String> = Vec::with_capacity(2);
            for (key, value) in params {
                if key == "code" || key == "state" {
                    params_values.push(value.to_string());
                }
            }

            if params_values.len() != 2 {
                // User denied consent
                let params_response = "denied".to_string();
                tx.send(params_response).await?;

                let response = Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(Body::empty())?;
                return Ok(response);
            }

            // User granted consent. Send authorization code back
            let params_response = format!("ok {} {}", params_values[0], params_values[1]);
            tx.send(params_response).await?;

            let response = Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())?;

            Ok(response)
        }
        _ => {
            let params_response = "error".to_string();
            tx.send(params_response).await?;

            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())?;

            Ok(response)
        }
    }
}

pub async fn http_server_get_params() -> Result<String> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(1);

    let server_fn_gen = |tx: mpsc::Sender<String>| {
        service_fn(move |req: Request<Body>| {
            let tx_clone = tx.clone();
            handle_callback(req, tx_clone)
        })
    };

    let service = make_service_fn(move |_socket: &AddrStream| {
        let tx_clone = tx.clone();
        async move { Ok::<_, hyper::Error>(server_fn_gen(tx_clone)) }
    });

    let _handle: tokio::task::JoinHandle<Result<(), anyhow::Error>> = tokio::task::spawn(async {
        let addr = ([127, 0, 0, 1], 8000).into();
        let server = Server::bind(&addr).serve(service);
        match server.await {
            Ok(_) => Ok(()),
            Err(_) => anyhow::bail!(display_error_info("Local HTTP Server binding failed.")),
        }
    });
    // Receive authorization code and csrf state from HTTP server
    let params = tokio::task::spawn(async move {
        match rx.recv().await {
            Some(values) => values,
            None => {
                log::debug!("Sender side of the channel has been closed. Cannot read any values on the receiver side.");
                "error".to_string()
            },
        }
    })
    .await?;
    Ok(params)
}
