use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;


#[derive(Clone)]
pub struct EmojiReplacements {
    pub emojis:HashMap<String,String>,
}

impl  EmojiReplacements {
    pub fn load_emojis(&mut self, path:String) ->Result<bool,&'static str> {
        
        let emoji_file = File::open(path).ok().unwrap();
        let emoji_reader = BufReader::new(emoji_file);
        let res = serde_json::from_reader(emoji_reader);
        if res.is_err() {
            panic!("Could not parse emoji file")
        }else{
            let emojis:serde_json::Value  = res.unwrap();
            //Read in all the emojis from the JSON
            for emoji in emojis.as_array().expect("Failed to load array of emojis in file") {

                //Try to parse out the glyph via the unified attribute which is a set of chars with - between them
                    let mut valid = true;
                    let mut unichar :Vec<char> = vec![];
                    for un in emoji["unified"].to_string().replace("\"","").split("-") {
                       // Just in case something invalid is added
                       let parse_res = u32::from_str_radix(un,16);
                       if parse_res.is_err() {
                            valid = false;
                       }else{
                        unichar.push( char::from_u32(parse_res.expect("Shouldn't occur, error parsing Unicode") ).expect("Failed to convert from U32 to char when loading emojis"));
                       }
                        
                    }

                    // Load in the unicode string
                    if valid {
                        let unicode_glyph = (unichar.into_iter()).collect::<String>();
                        &self.emojis.insert(emoji["short_name"].to_string(),unicode_glyph);
                    }

            }

            Ok(true)
        }


        
    }

    pub fn replace_emojis(&self,source_string:String)  ->Result<String,&'static str> {
        let mut message = source_string.clone();
        for (key,emoji) in &self.emojis {
            message = message.replace(key, emoji);
        }

        Ok(message)
       
    }

}

#[derive(Deserialize, Debug)]
struct JsonEmoji {
    name: String,
    unified: String,
    short_name: String,
    non_qualified: String,
    short_names: Vec<String>,
    docomo: String,
    au: String,
    google: String,
    softbank: String,
    image: String,
    sheet_x: u32,
    sheet_y: u32,
    sort_order: u32,
    text: String,
    texts: Vec<String>,
    category: String,
    subcategory: String,
    added_in: String,
    has_img_apple: bool,
    has_img_google: bool,
    has_img_twitter: bool,
    has_img_facebook: bool,
}