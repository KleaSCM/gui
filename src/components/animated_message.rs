// This file is intentionally empty 

use iced::{
    widget::{container, row, text, Column, mouse_area},
    Element, Length, Padding, Theme,
};
use std::time::Duration;
use crate::components::{
    animation::{Animation, AnimationType},
    styles::{AssistantMessageStyle, UserMessageStyle},
};

#[derive(Debug, Clone)]
pub struct AnimatedMessage {
    pub message: String,
    pub role: String,
    pub timestamp: String,
    pub status: String,
    pub animation: Animation,
}

impl AnimatedMessage {
    pub fn new(message: String, role: String) -> Self {
        let message_len = message.len();
        let animation_type = match role.as_str() {
            "assistant" => AnimationType::Typing,
            "system" => AnimationType::FadeIn,
            _ => AnimationType::SlideIn,
        };
        
        Self {
            message,
            role,
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
            status: "Sent".to_string(),
            animation: Animation::new(animation_type, Duration::from_millis(50 * message_len as u64)),
        }
    }

    pub fn update(&mut self) {
        self.animation.update();
    }

    pub fn is_complete(&self) -> bool {
        self.animation.is_complete
    }

    pub fn view<'a>(&self) -> Element<'a, Theme> {
        let message_style = if self.role == "user" {
            iced::theme::Container::Custom(Box::new(UserMessageStyle))
        } else {
            iced::theme::Container::Custom(Box::new(AssistantMessageStyle))
        };

        let content = Column::new()
            .push(text(&self.message))
            .push(
                row![
                    text(&self.timestamp).size(12),
                    text(&self.status).size(12),
                ]
                .spacing(8),
            )
            .spacing(8)
            .padding(Padding::from(8))
            .width(Length::Shrink);

        let message_container = mouse_area(
            container(content)
                .style(message_style)
                .padding(Padding::from(8))
                .width(Length::Shrink)
        )
        .on_press(Theme::default());

        row![
            super::avatar::view_avatar::<Theme>(&self.role),
            message_container
        ]
        .spacing(8)
        .padding(Padding::from(8))
        .width(Length::Fill)
        .into()
    }
} 