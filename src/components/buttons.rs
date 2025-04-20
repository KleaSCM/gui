use iced::{
    widget::{button, row, tooltip, text},
    Element, Length, Color,
};

use super::styles::CyberpunkButtonStyle;

pub fn circular_button<'a, MessageType: 'static + Clone>(
    icon: &'a str,
    tooltip_text: &'a str,
    on_press: MessageType,
) -> Element<'a, MessageType> {
    let button = button(
        text(icon)
            .size(20)
            .style(iced::theme::Text::Color(Color::WHITE))
    )
    .padding(10)
    .width(Length::Fixed(40.0))
    .height(Length::Fixed(40.0))
    .on_press(on_press)
    .style(iced::theme::Button::Custom(Box::new(CyberpunkButtonStyle)));

    tooltip(button, tooltip_text, tooltip::Position::Bottom)
        .style(iced::theme::Container::Custom(Box::new(super::styles::TooltipStyle)))
        .into()
}

pub fn circular_button_row<'a, MessageType: 'static + Clone>(
    buttons: Vec<Element<'a, MessageType>>,
) -> Element<'a, MessageType> {
    row(buttons)
        .spacing(10)
        .align_items(iced::Alignment::Center)
        .into()
} 