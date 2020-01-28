use actix_web::body::Body;
use rust_embed::{RustEmbed};
use std::borrow::Cow;
use actix_web::{App, HttpServer, HttpResponse, Error, get, web, HttpRequest, middleware};
use mime_guess::from_path;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

/// Define http actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}


#[derive(RustEmbed)]
#[folder = "target/deploy/"]
struct Asset;

#[get("/ws/")]
async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok().content_type(from_path(path).first_or_octet_stream().as_ref()).body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}
#[get("/{file}")]
async fn get_file(info: web::Path<String>) -> HttpResponse {
    handle_embedded_file(&info)
}

#[get("/")]
async fn root(_req: HttpRequest) -> HttpResponse {
    handle_embedded_file("index.html")
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
            .wrap(middleware::Logger::default())
            .service(websocket)
            .service(root)
            .service(get_file))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

