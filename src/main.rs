use iced::{
    widget::{button, column, container, row, scrollable, text_input, text},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};

mod components;
use components::{
    drop_zone::DropZone,
    animated_message::AnimatedMessage,
};

#[derive(Debug, Default)]
struct ChatApp {
    animated_messages: Vec<AnimatedMessage>,
    input: String,
    show_history: bool,
    is_typing: bool,
    show_scroll_button: bool,
    drop_zone: DropZone,
    scroll_offset: f32,
    animation_progress: f32,
}

#[derive(Debug, Clone)]
enum MessageEvent {
    InputChanged(String),
    SendMessage,
    ToggleHistory,
    Search,
    ScrollToBottom,
    AnimationTick,
    TypingAnimationTick,
    AddAssistantMessage,
}

impl Application for ChatApp {
    type Message = MessageEvent;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<MessageEvent>) {
        (
            Self {
                animation_progress: 0.0,
                ..Default::default()
            },
            Command::batch(vec![
                Command::perform(
                    async {
                        tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
                    },
                    |_| MessageEvent::AnimationTick,
                ),
                Command::perform(
                    async {
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    },
                    |_| MessageEvent::TypingAnimationTick,
                ),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Shandris")
    }

    fn update(&mut self, message: MessageEvent) -> Command<MessageEvent> {
        match message {
            MessageEvent::InputChanged(input) => {
                self.input = input;
                Command::none()
            }
            MessageEvent::SendMessage => {
                if !self.input.trim().is_empty() {
                    let animated_msg = AnimatedMessage::new(self.input.clone(), "user".to_string());
                    self.animated_messages.push(animated_msg);
                    self.input.clear();
                    self.is_typing = true;
                    self.show_scroll_button = true;
                    self.scroll_offset = f32::MAX;
                    self.animation_progress = 0.0;
                    
                    Command::batch(vec![
                        Command::perform(
                            async {
                                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                            },
                            |_| MessageEvent::AddAssistantMessage
                        ),
                        Command::perform(
                            async {
                                tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
                            },
                            |_| MessageEvent::AnimationTick
                        ),
                        Command::perform(
                            async {},
                            |_| MessageEvent::AnimationTick
                        ),
                    ])
                } else {
                    Command::none()
                }
            }
            MessageEvent::AddAssistantMessage => {
                let assistant_msg = AnimatedMessage::new(
                    "This is a simulated response.".to_string(),
                    "assistant".to_string()
                );
                self.animated_messages.push(assistant_msg);
                Command::perform(
                    async {
                        tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
                    },
                    |_| MessageEvent::AnimationTick,
                )
            }
            MessageEvent::ToggleHistory => {
                self.show_history = !self.show_history;
                Command::none()
            }
            MessageEvent::Search => {
                println!("Search clicked");
                Command::none()
            }
            MessageEvent::ScrollToBottom => {
                self.show_scroll_button = false;
                self.scroll_offset = f32::MAX;
                Command::none()
            }
            MessageEvent::AnimationTick => {
                let mut all_complete = true;
                for message in &mut self.animated_messages {
                    message.update();
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
            MessageEvent::TypingAnimationTick => {
                if self.is_typing {
                    Command::perform(
                        async {
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                        },
                        |_| MessageEvent::TypingAnimationTick,
                    )
                } else {
                    Command::none()
                }
            }
        }
    }

    fn view(&self) -> Element<MessageEvent> {
        let messages = self.animated_messages.iter().map(|msg| {
            msg.view().map(|_| MessageEvent::AnimationTick)
        });
        let messages_column = column(messages).spacing(10);

        let typing_indicator = if self.is_typing {
            let dots = (self.animation_progress * 3.0).floor() as usize;
            let dots_text = ".".repeat(dots);
            container(
                text(format!("Assistant is typing{}", dots_text))
                    .style(iced::theme::Text::Color(iced::Color::from_rgb(0.7, 0.7, 0.7)))
            )
            .padding(10)
            .into()
        } else {
            Element::new(iced::widget::Space::new(Length::Fixed(0.0), Length::Fixed(0.0)))
        };

        let scroll_button = if self.show_scroll_button {
            container(components::scroll_button::scroll_button(MessageEvent::ScrollToBottom))
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
                .width(Length::Fill)
                .on_scroll(|_| MessageEvent::AnimationTick),
            typing_indicator,
            row![input, send_button]
                .spacing(10)
                .padding(10)
                .align_items(Alignment::Center),
            self.drop_zone.view(),
            scroll_button
        ]
        .spacing(10)
        .padding(20);

        let main_content = if self.show_history {
            row![
                components::avatar_pane::view_avatar_pane::<MessageEvent>(MessageEvent::ToggleHistory, MessageEvent::Search),
                chat_content,
                components::history::view_history::<MessageEvent>(&self.animated_messages)
            ]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill)
        } else {
            row![
                components::avatar_pane::view_avatar_pane::<MessageEvent>(MessageEvent::ToggleHistory, MessageEvent::Search),
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
