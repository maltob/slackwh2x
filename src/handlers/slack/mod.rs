
use crate::handlers::handler;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;

pub struct SlackHandler{
    pub url: String,
}
 impl handler::MessageHandler for SlackHandler {
  fn   send (&self, msg:&crate::SlackMessage) -> Result<String, &'static str> {
      //Send the message on to Slack
        let client = Client::new();
        let res = client.post(&self.url)
            .body(serde_json::to_string(msg).unwrap())
            .send();
        
        println!("{}",res.unwrap().status());
        Ok("".to_string())
    }

}