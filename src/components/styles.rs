use iced::{Theme, widget::{container, button, text_input, text}};

pub struct UserMessageStyle;
pub struct AssistantMessageStyle;
pub struct CyberpunkBackgroundStyle;
pub struct CyberpunkInputStyle;
pub struct CyberpunkButtonStyle;
pub struct HistoryPaneStyle;
pub struct HistoryMessageStyle;
pub struct AvatarStyle;
pub struct AvatarPaneStyle;
pub struct TooltipStyle;
pub struct GlowingTextStyle;

impl container::StyleSheet for UserMessageStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.4, 0.2, 0.5))), // Soft purple
            text_color: Some(iced::Color::WHITE),
            border: iced::Border {
                radius: 15.0.into(),
                width: 1.0,
                color: iced::Color::from_rgb(0.6, 0.3, 0.7), // Lighter purple border
            },
            ..Default::default()
        }
    }
}

impl container::StyleSheet for AssistantMessageStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.2, 0.3, 0.4))), // Soft teal
            text_color: Some(iced::Color::WHITE),
            border: iced::Border {
                radius: 15.0.into(),
                width: 1.0,
                color: iced::Color::from_rgb(0.3, 0.4, 0.5), // Lighter teal border
            },
            ..Default::default()
        }
    }
}

impl container::StyleSheet for CyberpunkBackgroundStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.1, 0.1, 0.15))), // Dark background
            ..Default::default()
        }
    }
}

impl button::StyleSheet for CyberpunkButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.4, 0.2, 0.5))),
            text_color: iced::Color::WHITE,
            border: iced::Border {
                radius: 50.0.into(),
                width: 1.0,
                color: iced::Color::from_rgb(0.6, 0.3, 0.7),
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgb(0.4, 0.2, 0.5),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.5, 0.3, 0.6))),
            text_color: iced::Color::WHITE,
            border: iced::Border {
                radius: 50.0.into(),
                width: 1.0,
                color: iced::Color::from_rgb(0.7, 0.4, 0.8),
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgb(0.6, 0.3, 0.7),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        }
    }
}

impl text_input::StyleSheet for CyberpunkInputStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(iced::Color::from_rgb(0.15, 0.15, 0.2)), // Dark input
            border: iced::Border {
                radius: 8.0.into(),
                width: 1.0,
                color: iced::Color::from_rgb(0.4, 0.2, 0.5), // Purple border
            },
            icon_color: iced::Color::WHITE,
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(iced::Color::from_rgb(0.2, 0.2, 0.25)), // Slightly lighter
            border: iced::Border {
                radius: 8.0.into(),
                width: 1.0,
                color: iced::Color::from_rgb(0.6, 0.3, 0.7), // Brighter border
            },
            icon_color: iced::Color::WHITE,
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        iced::Color::from_rgb(0.5, 0.5, 0.6)
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        iced::Color::WHITE
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        iced::Color::from_rgb(0.4, 0.2, 0.5)
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        iced::Color::from_rgb(0.4, 0.4, 0.5)
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(iced::Color::from_rgb(0.1, 0.1, 0.15)),
            border: iced::Border {
                radius: 8.0.into(),
                width: 1.0,
                color: iced::Color::from_rgb(0.3, 0.2, 0.4),
            },
            icon_color: iced::Color::from_rgb(0.6, 0.6, 0.7),
        }
    }
}

impl container::StyleSheet for HistoryPaneStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.15, 0.15, 0.2))),
            border: iced::Border {
                radius: 15.0.into(),
                width: 2.0,
                color: iced::Color::from_rgb(0.4, 0.2, 0.5),
            },
            ..Default::default()
        }
    }
}

impl container::StyleSheet for HistoryMessageStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.2, 0.2, 0.25))),
            text_color: Some(iced::Color::WHITE),
            border: iced::Border {
                radius: 15.0.into(),
                width: 1.0,
                color: iced::Color::from_rgb(0.5, 0.3, 0.6),
            },
            ..Default::default()
        }
    }
}

impl container::StyleSheet for AvatarStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.2, 0.2, 0.25))),
            border: iced::Border {
                radius: 50.0.into(),
                width: 3.0,
                color: iced::Color::from_rgb(0.4, 0.2, 0.5),
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgb(0.4, 0.2, 0.5),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        }
    }
}

impl container::StyleSheet for AvatarPaneStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgba(0.15, 0.15, 0.2, 0.9))),
            border: iced::Border {
                radius: 0.0.into(),
                width: 0.0,
                color: iced::Color::TRANSPARENT,
            },
            ..Default::default()
        }
    }
}

impl container::StyleSheet for TooltipStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.2, 0.1, 0.3))),
            text_color: Some(iced::Color::WHITE),
            border: iced::Border {
                radius: 8.0.into(),
                width: 1.0,
                color: iced::Color::from_rgb(0.4, 0.2, 0.5),
            },
            ..Default::default()
        }
    }
}

impl text::StyleSheet for GlowingTextStyle {
    type Style = Theme;

    fn appearance(&self, _style: Theme) -> text::Appearance {
        text::Appearance {
            color: Some(iced::Color::WHITE),
        }
    }
} 