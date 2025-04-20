use iced::{
    widget::{container, image},
    Element, Length,
};

use super::styles::AvatarStyle;

pub fn view_avatar<'a, MessageType: 'static>(role: &str) -> Element<'a, MessageType> {
    let avatar_path = if role == "user" {
        "assets/5.jpg"
    } else {
        "assets/5jpg"
    };

    container(
        image::Image::new(avatar_path)
            .width(Length::Fixed(40.0))
            .height(Length::Fixed(40.0))
    )
    .padding(5)
    .style(iced::theme::Container::Custom(Box::new(AvatarStyle)))
    .into()
} 