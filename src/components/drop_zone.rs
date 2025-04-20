use iced::{
    widget::{container, text},
    Element, Length, Command,
};
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct DropZone {
    pub is_dragging: bool,
    pub image_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DropZoneMessage {
    DragEnter,
    DragLeave,
    FileDropped(PathBuf),
}

impl DropZone {
    pub fn update(&mut self, message: DropZoneMessage) -> Command<DropZoneMessage> {
        match message {
            DropZoneMessage::DragEnter => {
                self.is_dragging = true;
                Command::none()
            }
            DropZoneMessage::DragLeave => {
                self.is_dragging = false;
                Command::none()
            }
            DropZoneMessage::FileDropped(path) => {
                self.is_dragging = false;
                self.image_path = Some(path);
                Command::none()
            }
        }
    }

    pub fn view<'a>(&self) -> Element<'a, DropZoneMessage> {
        let content = if let Some(path) = &self.image_path {
            Element::from(text(format!("Image dropped: {}", path.display())).style(iced::theme::Text::Default))
        } else {
            let text_content = if self.is_dragging {
                text("Drop image here...").style(iced::theme::Text::Default)
            } else {
                text("Drag and drop an image here").style(iced::theme::Text::Default)
            };
            Element::from(text_content)
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fixed(100.0))
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(DropZoneStyle {
                is_dragging: self.is_dragging,
            })))
            .into()
    }
}

struct DropZoneStyle {
    is_dragging: bool,
}

impl iced::widget::container::StyleSheet for DropZoneStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(if self.is_dragging {
                iced::Color::from_rgb(0.4, 0.2, 0.5)
            } else {
                iced::Color::from_rgb(0.2, 0.2, 0.25)
            })),
            border: iced::Border {
                radius: 15.0.into(),
                width: 2.0,
                color: if self.is_dragging {
                    iced::Color::from_rgb(0.6, 0.3, 0.7)
                } else {
                    iced::Color::from_rgb(0.4, 0.2, 0.5)
                },
            },
            ..Default::default()
        }
    }
} 