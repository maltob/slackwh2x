mod mapping;
mod handlers;
mod emoji_replacement;

use actix_web::{ post, web, App, HttpResponse, HttpServer, Responder};
use mapping::{run_handlers};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;
use std::collections::HashMap;

// Take in posts for a webhook ase defined in the slack2x.toml
#[post("/{name}")]
async fn post_message(name: web::Path<(String,)>, body: web::Bytes, data: web::Data<AppState>) -> impl Responder {
    println!("{}",std::str::from_utf8(&body).unwrap());
    let result:SlackMessage = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
    println!("{}",result.text);
    let emoji_handler = &*data.emojis.read().ok().unwrap();
    let handlers = run_handlers(&name.into_inner().0.as_str(), result, emoji_handler);
    match handlers {
        Ok(h) => {HttpResponse::Ok().body( h )}
        Err(v) => {HttpResponse::NotFound().body(v)}
    }
    

    
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let state = web::Data::new( AppState { emojis:RwLock::new(emoji_replacement::EmojiReplacements { emojis: HashMap::new() })});
    state.emojis.write().unwrap().load_emojis("./data/emoji/emoji.json".to_string()).ok();

    HttpServer::new(move || {
        App::new().app_data(state.clone()).service(post_message)
           
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

struct AppState {
    pub emojis: RwLock<emoji_replacement::EmojiReplacements>,
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