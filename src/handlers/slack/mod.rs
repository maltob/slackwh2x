
use crate::handlers::handler;
use reqwest::blocking::Client;
use crate::emoji_replacement::EmojiReplacements;

pub struct SlackHandler{
    pub url: String,
}
 impl handler::MessageHandler for SlackHandler {
  fn   send (&self, msg:&crate::SlackMessage,emoji_rep:&EmojiReplacements) -> Result<String, &'static str> {
      //Send the message on to Slack
        let client = Client::new();
        let res = client.post(&self.url)
            .body(serde_json::to_string(msg).unwrap())
            .send();
        
        println!("{}",res.unwrap().status());
        Ok("".to_string())
    }

}