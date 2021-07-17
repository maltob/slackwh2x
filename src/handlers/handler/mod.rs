
   pub trait MessageHandler {
        fn send(&self, msg: &crate::SlackMessage) -> Result<String, &'static str>;
    }
