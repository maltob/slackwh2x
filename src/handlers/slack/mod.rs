
use crate::handlers::handler;
use reqwest::blocking::Client;
use crate::emoji_replacement::EmojiReplacements;
use log::{debug};

pub struct SlackHandler{
    pub url: String,
}
 impl handler::MessageHandler for SlackHandler {
  fn   send (&self, msg:&crate::SlackMessage,_emoji_rep:&EmojiReplacements) -> Result<String, &'static str> {
      //Send the message on to Slack
        let client = Client::new();
        let res = client.post(&self.url)
            .body(serde_json::to_string(msg).unwrap())
            .send();
        
        debug!("Slack message forwarded:\n{}",res.unwrap().status());
        Ok("".to_string())
    }

}