
use toml::{Value,map::Map};
use std::fs;
use crate::handlers::{handler,slack,teams};
use crate::handlers::handler::MessageHandler;

fn load_file(filename:&str) ->Result<String, &'static str> {
 match fs::read_to_string(filename) {
     Ok(contents) => Ok(contents),
     Err(c) => Err("")
 }
}

// Check for the mapping in the slack2x.toml file
pub fn lookup_key(key:&str) -> Result<String, &'static str> {
    let value = load_file("slack2x.toml")?.parse::<Value>().ok().unwrap();
    if value.is_table() {
        let  values:&Map<String, Value> = value.as_table().unwrap();
        
       if  values.contains_key(key) {
        Ok(String::from(values[key].as_str().unwrap()))
       }else{
        Err("Invalid Key")
       }
        
        
    }else{
        Err("")
    }
}


// Check each url if there are multiple - TODO - Check what happens when there are not multiple
pub fn run_handlers(key:&str, msg:crate::SlackMessage) -> Result<String, &'static str> {
    let urls = lookup_key(key).unwrap();
    
    for url in urls.split(";") {
        handle_url(url,&msg);
        println!("Handling {}",url);
    };

    Ok("".to_string())
}

//Handle the URL with the current basic URL detection
fn handle_url(url:&str, msg:&crate::SlackMessage) {
    if url.contains("https://hooks.slack.com/services") {
        //Slack
        let slackh = slack::SlackHandler {url:url.to_string()};
        slackh.send(&msg);
    }else if url.contains(".webhook.office.com/webhookb2") {
        //Teams
        let teamsh = teams::TeamsHandler {url:url.to_string()};
        teamsh.send(&msg);
    }
}