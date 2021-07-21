use crate::emoji_replacement::EmojiReplacements;

   pub trait MessageHandler {
        fn send(&self, msg: &crate::SlackMessage,emoji_rep:&EmojiReplacements) -> Result<String, &'static str>;
    }
