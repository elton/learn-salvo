use salvo::extra::cors::CorsHandler;
use salvo::extra::logging::LogHandler;
use salvo::http::header::{self, HeaderValue};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}

#[handler]
async fn hello_world() -> Result<&'static str, ()> {
    Ok("Hello World!")
}

#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) {
    tracing::info!(
        "remote address: {:?}",
        req.remote_addr().unwrap().as_ipv4().unwrap().ip()
    );
    // tracing::info!("headers: {:?}", req.headers());

    let user = User {
        id: 1,
        username: "admin".to_string(),
        password: "password".to_string(),
    };

    // res.render(serde_json::to_string(&user).unwrap());
    res.render(Text::Json(serde_json::to_string(&user).unwrap()));
}

#[handler]
async fn add_header(res: &mut Response) {
    res.headers_mut()
        .insert(header::SERVER, HeaderValue::from_static("Salvo"));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    tracing::info!("Listening on http://127.0.0.1:7878");

    Server::new(TcpListener::bind("127.0.0.1:7878"))
        .serve(route())
        .await;
}

fn route() -> Router {
    let cors_handler = CorsHandler::builder()
        // .with_allow_origin("http://localhost:7878")
        // .with_allow_methods(vec!["GET", "POST", "DELETE"])
        // .build();
        .with_allow_any_origin()
        .build();

    Router::with_hoop(cors_handler)
        .hoop(LogHandler)
        .hoop(add_header)
        .get(hello_world)
        .push(Router::with_path("users").get(get_user))
}
