use iced::{
    widget::{container, text, row, column},
    Element, Color,
};

use super::{styles::{UserMessageStyle, AssistantMessageStyle}, avatar::view_avatar, message::Message};

#[derive(Debug)]
pub struct AnimatedMessage {
    pub message: Message,
    pub animation_progress: f32,
}

impl AnimatedMessage {
    pub fn new(message: Message) -> Self {
        Self {
            message,
            animation_progress: 0.0,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.animation_progress = (self.animation_progress + delta).min(1.0);
    }

    pub fn is_complete(&self) -> bool {
        self.animation_progress >= 1.0
    }

    pub fn view<'a, MessageType: 'static>(&self) -> Element<'a, MessageType> {
        let timestamp = self.message.timestamp.format("%H:%M").to_string();
        let status_icon = match self.message.status {
            super::message::MessageStatus::Sent => "✓",
            super::message::MessageStatus::Delivered => "✓✓",
        };

        let message_content = column![
            text(&self.message.content)
                .style(iced::theme::Text::Color(Color::WHITE)),
            row![
                text(timestamp)
                    .size(12)
                    .style(iced::theme::Text::Color(Color::from_rgb(0.7, 0.7, 0.7))),
                text(status_icon)
                    .size(12)
                    .style(iced::theme::Text::Color(Color::from_rgb(0.7, 0.7, 0.7)))
            ]
            .spacing(5)
            .align_items(iced::Alignment::Center)
        ]
        .spacing(5);

        let message_style = if self.message.role == "user" {
            container(message_content)
                .padding(10)
                .style(iced::theme::Container::Custom(Box::new(UserMessageStyle)))
        } else {
            container(message_content)
                .padding(10)
                .style(iced::theme::Container::Custom(Box::new(AssistantMessageStyle)))
        };

        row![
            view_avatar::<MessageType>(&self.message.role),
            message_style
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center)
        .into()
    }
} 