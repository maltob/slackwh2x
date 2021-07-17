
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
mod mapping;
mod handlers;

use mapping::{run_handlers, lookup_key};
use serde::{Deserialize, Serialize};

// Take in posts for a webhook ase defined in the slack2x.toml
#[post("/{name}")]
async fn post_message(name: web::Path<(String,)>, body: web::Bytes) -> impl Responder {
    println!("{}",std::str::from_utf8(&body).unwrap());
    let result:SlackMessage = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
    println!("{}",result.text);
    HttpResponse::Ok().body( run_handlers(&name.into_inner().0.as_str(), result).unwrap() )

    
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new().service(post_message)
           
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



#[derive(Serialize, Deserialize)]
pub struct SlackMessage {
    channel: Option<String>,
    text: String,
    username: Option<String>,
    icon_emoji: Option<String>,
    attachments: Option<Vec<SlackAttachment>>,
}

#[derive(Serialize, Deserialize,  Clone)]
pub struct SlackAttachment {
    fallback: Option<String>,
    color: Option<String>,
    pretext: Option<String>,
    author_name: Option<String>,
    author_link: Option<String>,
    author_icon: Option<String>,
    title: Option<String>,
    title_link: Option<String>,
    text: Option<String>,
    image_url: Option<String>,
    thumb_url: Option<String>,
    footer: Option<String>,
    footer_icon: Option<String>,
    ts: Option<i64>,
    fields: Option<Vec<SlackField>>,

}

#[derive(Serialize, Deserialize, Clone)]
pub struct SlackField {
    title: String,
    value: String,
    short: bool,
}