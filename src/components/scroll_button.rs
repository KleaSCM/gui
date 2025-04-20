use iced::{
    widget::{button, text},
    Element, Length,
};

use super::styles::CyberpunkButtonStyle;

pub fn scroll_button<'a, MessageType: 'static + Clone>(
    on_scroll: MessageType,
) -> Element<'a, MessageType> {
    button(
        text("↓")
            .size(20)
            .style(iced::theme::Text::Color(iced::Color::WHITE))
    )
    .padding(10)
    .width(Length::Fixed(40.0))
    .height(Length::Fixed(40.0))
    .on_press(on_scroll)
    .style(iced::theme::Button::Custom(Box::new(CyberpunkButtonStyle)))
    .into()
} 