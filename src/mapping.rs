
use yaml_rust::{YamlLoader, YamlEmitter};
use std::fs;
use crate::handlers::{slack,teams};
use crate::handlers::handler::MessageHandler;
use crate::emoji_replacement::EmojiReplacements;

fn load_file(filename:&str) ->Result<String, &'static str> {
 match fs::read_to_string(filename) {
     Ok(contents) => Ok(contents),
     Err(_) => Err("")
 }
}

// Check for the mapping in the slack2x.toml file
pub fn lookup_key(key:&str) -> Result<Vec<String>, &'static str> {
    let yaml_conf = fs::read_to_string("slack2x.yaml").expect("Failed to read slack2x.yaml");
    let docs = YamlLoader::load_from_str(&yaml_conf).expect("Invalid YAML config");
    let doc = docs[0].clone();
        
       if  !doc[key].is_badvalue() {
           let mut urls = vec![];
           if doc[key].is_array() {
               for item in doc[key].as_vec().expect(format!("Not an array, invalid config for {}", key).as_str()) {
                   
                    urls.push(item.as_str().unwrap().to_string())
               }
           }
        Ok(urls)
       }else{
        Err("Invalid Key")
       }
        
        
    
}


// Check each url if there are multiple - TODO - Check what happens when there are not multiple
pub fn run_handlers(key:&str, msg:crate::SlackMessage, emoji_rep:&EmojiReplacements) -> Result<String, &'static str> {
    let res = lookup_key(key);
    match res {
        Ok(urls) => {
            for url in urls {
                handle_url(&url,&msg,emoji_rep);
            };
        
            Ok("Sent".to_string())
        },
        Err(_) => {Err("Key not found")}
    }
    
    
}

//Handle the URL with the current basic URL detection
fn handle_url(url:&str, msg:&crate::SlackMessage,emoji_rep:&EmojiReplacements) {
    if url.contains("https://hooks.slack.com/services") {
        //Slack
        let slackh = slack::SlackHandler {url:url.to_string()};
        slackh.send(&msg,emoji_rep).ok();
    }else if url.contains(".webhook.office.com/webhookb2") {
        //Teams
        let teamsh = teams::TeamsHandler {url:url.to_string()};
        teamsh.send(&msg,emoji_rep).ok();
    }
}
