mod mapping;
mod handlers;
mod emoji_replacement;

use actix_web::{ post, web, App, HttpResponse, HttpServer, Responder};
use mapping::{run_handlers};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;
use std::collections::HashMap;
use log::{debug, info};
use env_logger::Env;
use std::env;

// Take in posts for a webhook ase defined in the slack2x.toml
#[post("/{name}")]
async fn post_message(name: web::Path<(String,)>, body: web::Bytes, data: web::Data<AppState>) -> impl Responder {
    debug!("{}",std::str::from_utf8(&body).unwrap());
    let result:SlackMessage = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
    debug!("{}",result.text);
    let emoji_handler = &*data.emojis.read().ok().unwrap();
    let handlers = run_handlers(&name.into_inner().0.as_str(), result, emoji_handler);
    match handlers {
        Ok(h) => {debug!("{}",h);HttpResponse::Ok().body( h )},
        Err(v) => {debug!("{}",v);HttpResponse::NotFound().body(v)}
    }
    

    
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Loading emoji data");
    let state = web::Data::new( AppState { emojis:RwLock::new(emoji_replacement::EmojiReplacements { emojis: HashMap::new() })});
    state.emojis.write().unwrap().load_emojis("./data/emoji/emoji.json".to_string()).ok();

    // Load in the binding ports and addresses
    let bindv4 = env::var("BIND_ADDR").unwrap_or("0.0.0.0".to_string());
    let port = u16::from_str_radix(&env::var("BIND_PORT").unwrap_or("8080".to_string()),10).unwrap_or(8080);

    info!("Starting webserver on {}:{}",bindv4,port);
    HttpServer::new(move || {
        App::new().app_data(state.clone()).service(post_message)
           
    })
    .bind((bindv4, port))?
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