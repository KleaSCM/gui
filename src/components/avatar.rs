use iced::{
    widget::{container, text},
    Element,
};

use super::styles::AvatarStyle;

pub fn view_avatar<'a, MessageType: 'static>(role: &str) -> Element<'a, MessageType> {
    let emoji = if role == "user" { "👤" } else { "🤖" };
    
    container(
        text(emoji)
            .size(30)
    )
    .padding(5)
    .style(iced::theme::Container::Custom(Box::new(AvatarStyle)))
    .into()
} 