use iced::{
    widget::{container, image, column, text, vertical_space},
    Element, Length,
};

use super::styles::{AvatarPaneStyle, AvatarStyle};
use super::buttons::{circular_button, circular_button_row};

pub fn view_avatar_pane<'a, MessageType: 'static + Clone>(on_toggle_history: MessageType, on_search: MessageType) -> Element<'a, MessageType> {
    let avatar = container(
        image::Image::new("assets/5.jpg")
            .width(Length::Fixed(114.0))
            .height(Length::Fixed(114.0))
    )
    .width(Length::Fixed(120.0))
    .height(Length::Fixed(120.0))
    .center_x()
    .center_y()
    .style(iced::theme::Container::Custom(Box::new(AvatarStyle)));

    let buttons = circular_button_row(vec![
        circular_button("H", "History", on_toggle_history),
        circular_button("S", "Search", on_search),
    ]);

    let content = column![
        vertical_space(),
        avatar,
        vertical_space(),
        buttons,
        vertical_space(),
        text("Cyberpunk Chat")
            .size(24)
            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.8, 0.6, 0.9))),
        vertical_space(),
        text("v1.0")
            .size(14)
            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.6, 0.4, 0.7))),
        vertical_space()
    ]
    .spacing(5)
    .align_items(iced::Alignment::Center)
    .padding(20);

    container(content)
        .width(Length::Fixed(200.0))
        .height(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(AvatarPaneStyle)))
        .into()
} 