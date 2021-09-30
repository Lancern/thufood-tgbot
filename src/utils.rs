use teloxide::types::{Message, MessageKind, User};

/// Get the sender of the given Telegram message.
///
/// This function returns `None` if the given message is sent within a channel.
pub fn get_message_sender(message: &Message) -> Option<&User> {
    match &message.kind {
        MessageKind::Common(msg) => msg.from.as_ref(),
        _ => None,
    }
}

/// Get the display name of the user.
pub fn get_user_display_name(u: &User) -> String {
    match &u.username {
        Some(username) => format!("@{}", username),
        None => match &u.last_name {
            Some(last_name) => format!("{} {}", u.first_name, last_name),
            None => u.first_name.clone(),
        },
    }
}
