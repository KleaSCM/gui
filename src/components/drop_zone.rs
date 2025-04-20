use iced::{
    widget::{container, text},
    Element, Length, Color,
};

#[derive(Debug, Default)]
pub struct DropZone {
    pub is_dragging: bool,
}

impl DropZone {
    pub fn view<'a, MessageType: 'static>(&self) -> Element<'a, MessageType> {
        let content = if self.is_dragging {
            text("Drop image here...")
                .style(iced::theme::Text::Color(Color::WHITE))
        } else {
            text("Drag and drop an image here")
                .style(iced::theme::Text::Color(Color::from_rgb(0.7, 0.7, 0.7)))
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