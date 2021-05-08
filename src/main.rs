#[macro_use]
extern crate serde_derive;
use actix_web::{error, get, web, App, Error, HttpResponse, HttpServer};
use tera::Tera;

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[get("/{username}")]
async fn index(tmpl: web::Data<Tera>, info: web::Path<Info>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "テストタイトル");
    ctx.insert("message", "こんにちわ");
    ctx.insert("username", &info.username);
    let view = tmpl
        .render("index.html.tera", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let templates = Tera::new("templates/**/*").unwrap();

        App::new().data(templates).service(index)
    })
    .bind("127.0.0.1:3000")
    .expect("Can not bind to port 3000")
    .run()
    .await
}
