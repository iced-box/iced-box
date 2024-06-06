extern crate iced;
extern crate iced_box;

use iced::{
    event::{self, Event},
    Command,
    Element,
    Subscription,
    widget::{button, column},
};
use iced_box::toasts::{
    danger,
    Manager,
    primary,
    success,
    secondary,
    Toast,
};

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    IncrementTenPressed,
    DecrementTenPressed,
    Event(Event), // for toasts manager
    Close(usize), // for toasts manager
}

#[derive(Debug, Default)]
struct Counter {
    // The counter value
    value: i32,
    toasts: Vec<Toast>,
}

impl Counter {
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                self.toasts.push(success("Sucesso ao adicionar 1"));
            }
            Message::IncrementTenPressed => {
                self.value += 10;
                self.toasts.push(
                    secondary("Sucesso ao adicionar 10").body(
                        format!("O valor agora é {}", self.value).as_str()
                    )
                );
            }
            Message::DecrementPressed => {
                self.value -= 1;

                self.toasts.push(success("Removido 1"));
            },
            Message::DecrementTenPressed => {
                self.value += 10;
                self.toasts.push(
                    danger("Removido 10").body(
                        format!("O valor agora é {}", self.value).as_str()
                    )
                );
            }
            _ => {},
        }

        Command::none()
    }
    
    fn view(&self) -> Element<Message> {
        // We use a column: a simple vertical layout
        let content = column![
            // The increment button. We tell it to produce an
            // `IncrementPressed` message when pressed
            button("Adicionar 1").on_press(Message::IncrementPressed),
            button("Adicionar 10").on_press(Message::IncrementTenPressed),

            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button("Remover 1").on_press(Message::DecrementPressed),
            button("Remover 10").on_press(Message::DecrementTenPressed),
        ].spacing(20);

        Manager::new(content, &self.toasts, Message::Close)
            .timeout(5) // The alert will exist in 5 seconds
            .into()
    }
}
fn main() {
    let _ = iced::program(
        "Icex-box toasts",
        Counter::update,
        Counter::view
    ).subscription(
        Counter::subscription
    ).run();
}
