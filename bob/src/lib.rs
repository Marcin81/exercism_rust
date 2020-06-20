pub fn reply(message: &str) -> &str {
     let trimmed = message.trim();
     match trimmed {
         text if text.contains(char::is_uppercase) && text.find(char::is_lowercase).is_none() => {
             if text.ends_with("?") { "Calm down, I know what I'm doing!" } else { "Whoa, chill out!" }
         }
         text if text.ends_with("?") => "Sure.",
         text if text.is_empty() => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
