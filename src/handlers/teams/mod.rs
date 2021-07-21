
use crate::handlers::handler;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use crate::emoji_replacement::EmojiReplacements;

pub struct TeamsHandler{
   pub url: String,
}

impl  TeamsHandler {
    fn convert_to_teams_simple(&self,msg:&crate::SlackMessage) -> Result<SimpleTeamsMessage, &'static str> {
           Ok(SimpleTeamsMessage {text: msg.text.to_string()})
    }

    fn convert_to_teams_list(&self,msg:&crate::SlackMessage,emoji_rep:&EmojiReplacements) -> Result<MessageCardContent, &'static str> {
       
        let mut factset = vec![];

        

        if !msg.attachments.is_none() {
            let atts = msg.attachments.clone();
            for att in atts.unwrap().iter() {
               let flds = &att.fields.clone().unwrap();

                for field in flds {
                    {
                       
                        factset.push(MessageCardFact {name: field.title.to_string(), value:field.value.to_string() });
                        
                      }

                }
            }
        }

       
        let msg_body = MessageCardSection {title:" ".to_string(),facts:factset};

        // Text needs to have some length or teams will deny the card
        let msg_content = MessageCardContent {context:"http://schema.org/extensions".to_string(), ttype:"MessageCard".to_string(), sections:vec![msg_body], text:" ".to_string(), title:emoji_rep.replace_emojis(msg.text.to_string()).expect("Failed to replace emojis")};
        Ok(msg_content)
    }
    
   
}
//

 impl handler::MessageHandler for TeamsHandler {
    fn send (&self, msg:&crate::SlackMessage, emoji_rep:&EmojiReplacements) -> Result<String, &'static str> {
        let client = Client::new();

        //Handle just text based messages
        if msg.attachments.is_none()  {
            let teams_msg_res = self.convert_to_teams_simple(msg);
            match teams_msg_res {
                Ok(res_msg) => {
                    let _res = client.post(&self.url)
                    .body(serde_json::to_string(&res_msg).unwrap())
                    .send();
                    Ok("".to_string())
                },
                Err(_res_msg) => Err(""),
            }
    }else if !msg.attachments.is_none() {
        //Handle more advanced messages by converting them to O365 Connector Cards
        let teams_msg_res = self.convert_to_teams_list(msg, emoji_rep);
        
            match teams_msg_res {
                Ok(res_msg) => {
                    println!("Attachments! {}",serde_json::to_string_pretty(&res_msg).unwrap());
                    let res = client.post(&self.url)
                    .body(serde_json::to_string_pretty(&res_msg).unwrap())
                    .send();
                    println!("{}",res.unwrap().status());
                    Ok("".to_string())
                },
                Err(_res_msg) => Err(""),
            }
       
    }else{
        Err("")
    }
    
        
    }
}



#[derive(Serialize, Deserialize)]
struct SimpleTeamsMessage {
    
    text:String
}

#[derive(Serialize, Deserialize)]
struct ListTeamsMessage {
    #[serde (rename = "contentType")] 
    content_type:String,
    content:MessageCardContent,
}

#[derive(Serialize, Deserialize)]
struct MessageCardContent {
    #[serde (rename = "@context")] 
    context:String,
    #[serde (rename = "@type")] 
    ttype:String,
    sections:Vec<MessageCardSection>,
    text:String,
    title:String
}

#[derive(Serialize, Deserialize)]
struct MessageCardSection {
    title:String,
    facts:Vec<MessageCardFact>,
}

#[derive(Serialize, Deserialize)]
struct MessageCardFact {
    name:String,
    value:String,
}