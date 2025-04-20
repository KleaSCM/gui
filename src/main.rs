use iced::{
    widget::{button, column, container, row, scrollable, text_input, text},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};

mod components;
use components::{
    Message, history::view_history, avatar_pane::view_avatar_pane,
    animated_message::AnimatedMessage, scroll_button::scroll_button,
};
use components::message::MessageStatus;

#[derive(Debug, Default)]
struct ChatApp {
    messages: Vec<Message>,
    animated_messages: Vec<AnimatedMessage>,
    input: String,
    show_history: bool,
    is_typing: bool,
    show_scroll_button: bool,
}

#[derive(Debug, Clone)]
enum MessageEvent {
    InputChanged(String),
    SendMessage,
    ToggleHistory,
    Search,
    UpdateMessageStatus(usize, MessageStatus),
    ScrollToBottom,
    AnimationTick,
}

impl Application for ChatApp {
    type Message = MessageEvent;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<MessageEvent>) {
        (
            Self::default(),
            Command::perform(
                async {
                    tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
                },
                |_| MessageEvent::AnimationTick,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Cyberpunk Chat")
    }

    fn update(&mut self, message: MessageEvent) -> Command<MessageEvent> {
        match message {
            MessageEvent::InputChanged(input) => {
                self.input = input;
                Command::none()
            }
            MessageEvent::SendMessage => {
                if !self.input.trim().is_empty() {
                    let user_message = Message {
                        role: "user".to_string(),
                        content: self.input.clone(),
                        timestamp: chrono::Local::now(),
                        status: MessageStatus::Sent,
                    };
                    self.messages.push(user_message.clone());
                    self.animated_messages.push(AnimatedMessage::new(user_message));
                    self.input.clear();
                    self.is_typing = true;
                    self.show_scroll_button = true;
                    
                    let message_index = self.messages.len() - 1;
                    
                    Command::batch(vec![
                        Command::perform(
                            async {
                                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                            },
                            move |_| MessageEvent::UpdateMessageStatus(message_index, MessageStatus::Delivered)
                        ),
                        Command::perform(
                            async {
                                tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
                            },
                            |_| MessageEvent::AnimationTick,
                        ),
                    ])
                } else {
                    Command::none()
                }
            }
            MessageEvent::ToggleHistory => {
                self.show_history = !self.show_history;
                Command::none()
            }
            MessageEvent::Search => {
                println!("Search clicked");
                Command::none()
            }
            MessageEvent::UpdateMessageStatus(index, status) => {
                if let Some(message) = self.messages.get_mut(index) {
                    message.status = status.clone();
                    match status {
                        MessageStatus::Delivered => {
                            self.is_typing = false;
                            let assistant_message = Message {
                                role: "assistant".to_string(),
                                content: "This is a simulated response.".to_string(),
                                timestamp: chrono::Local::now(),
                                status: MessageStatus::Sent,
                            };
                            self.messages.push(assistant_message.clone());
                            self.animated_messages.push(AnimatedMessage::new(assistant_message));
                            self.show_scroll_button = true;
                        }
                        _ => {}
                    }
                }
                Command::perform(
                    async {
                        tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
                    },
                    |_| MessageEvent::AnimationTick,
                )
            }
            MessageEvent::ScrollToBottom => {
                self.show_scroll_button = false;
                Command::none()
            }
            MessageEvent::AnimationTick => {
                let mut all_complete = true;
                for message in &mut self.animated_messages {
                    message.update(0.1);
                    if !message.is_complete() {
                        all_complete = false;
                    }
                }

                if !all_complete {
                    Command::perform(
                        async {
                            tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
                        },
                        |_| MessageEvent::AnimationTick,
                    )
                } else {
                    Command::none()
                }
            }
        }
    }

    fn view(&self) -> Element<MessageEvent> {
        let messages = self.animated_messages.iter().map(|msg| msg.view::<MessageEvent>());

        let messages_column = column(messages).spacing(10);

        let typing_indicator = if self.is_typing {
            container(
                text("Assistant is typing...")
                    .style(iced::theme::Text::Color(iced::Color::from_rgb(0.7, 0.7, 0.7)))
            )
            .padding(10)
            .into()
        } else {
            Element::new(iced::widget::Space::new(Length::Fixed(0.0), Length::Fixed(0.0)))
        };

        let scroll_button = if self.show_scroll_button {
            container(scroll_button(MessageEvent::ScrollToBottom))
                .padding(10)
                .into()
        } else {
            Element::new(iced::widget::Space::new(Length::Fixed(0.0), Length::Fixed(0.0)))
        };

        let input = text_input("Type a message...", &self.input)
            .on_input(MessageEvent::InputChanged)
            .on_submit(MessageEvent::SendMessage)
            .padding(10)
            .style(iced::theme::TextInput::Custom(Box::new(components::styles::CyberpunkInputStyle)));

        let send_button = button("Send")
            .on_press(MessageEvent::SendMessage)
            .padding(10)
            .style(iced::theme::Button::Custom(Box::new(components::styles::CyberpunkButtonStyle)));

        let chat_content = column![
            scrollable(messages_column)
                .height(Length::Fill)
                .width(Length::Fill),
            typing_indicator,
            row![input, send_button]
                .spacing(10)
                .padding(10)
                .align_items(Alignment::Center),
            scroll_button
        ]
        .spacing(10)
        .padding(20);

        let main_content = if self.show_history {
            row![
                view_avatar_pane::<MessageEvent>(MessageEvent::ToggleHistory, MessageEvent::Search),
                chat_content,
                view_history::<MessageEvent>(&self.messages)
            ]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill)
        } else {
            row![
                view_avatar_pane::<MessageEvent>(MessageEvent::ToggleHistory, MessageEvent::Search),
                chat_content
            ]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill)
        };

        container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(components::styles::CyberpunkBackgroundStyle)))
            .into()
    }
}

fn main() -> iced::Result {
    ChatApp::run(Settings::default())
}
