use iced::{
    widget::{column, container, scrollable, text},
    Element, Length,
};

use super::styles::{HistoryMessageStyle, HistoryPaneStyle};

pub fn view_history<'a, MessageType: 'static>(messages: &[super::Message]) -> Element<'a, MessageType> {
    let history_content = column![
        text("Chat History").size(20),
        scrollable(
            column(
                messages
                    .iter()
                    .map(|msg| {
                        container(text(&msg.content))
                            .padding(10)
                            .style(iced::theme::Container::Custom(Box::new(HistoryMessageStyle)))
                            .into()
                    })
                    .collect::<Vec<_>>()
            )
            .spacing(5)
        )
        .height(Length::Fill)
    ]
    .spacing(10)
    .padding(10);

    container(history_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(HistoryPaneStyle)))
        .into()
} 