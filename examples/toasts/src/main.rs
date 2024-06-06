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
    IncrementFivePressed,
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
                self.toasts.push(success("Success when adding 1"));
            }
            Message::IncrementFivePressed => {
                self.value += 10;
                self.toasts.push(
                    primary("Added 5").body(
                        format!("The value is now {}", self.value).as_str()
                    ).with_close()
                );
            }
            Message::IncrementTenPressed => {
                self.value += 10;
                self.toasts.push(
                    secondary("Success in adding 10").body(
                        format!("The value is now {}", self.value).as_str()
                    )
                );
            }
            Message::DecrementPressed => {
                self.value -= 1;

                self.toasts.push(success("Removed 1"));
            },
            Message::DecrementTenPressed => {
                self.value += 10;
                self.toasts.push(
                    danger("Removed 10").body(
                        format!("The value is now {}", self.value).as_str()
                    )
                );
            }
            Message::Close(index) => {
                self.toasts.remove(index);
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
            button("Add 1 without body").on_press(Message::IncrementPressed),
            button("Add 5 with body and close button").on_press(Message::IncrementFivePressed),
            button("Add 10 with body").on_press(Message::IncrementTenPressed),

            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button("Remove 1 without body").on_press(Message::DecrementPressed),
            button("Remove 10 with body").on_press(Message::DecrementTenPressed),
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
