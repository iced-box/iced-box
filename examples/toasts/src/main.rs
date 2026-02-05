extern crate iced;
extern crate iced_box;

use iced::{
    Element, Task,
    widget::{button, column},
};
use iced_box::toasts::{Manager, Toast, danger, primary, secondary, success};

#[derive(Debug, Clone)]
pub enum Message {
    IncrementFivePressed,
    IncrementPressed,
    DecrementPressed,
    IncrementTenPressed,
    DecrementTenPressed,
    Close(usize), // for toasts manager
}

#[derive(Debug, Default)]
struct Counter {
    // The counter value
    value: i32,
    toasts: Vec<Toast>,
}

impl Counter {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                self.toasts.push(success("Success when adding 1"));
            }
            Message::IncrementFivePressed => {
                self.value += 5;
                self.toasts.push(
                    primary("Added 5")
                        .body(format!("The value is now {}", self.value).as_str())
                        .with_close(),
                );
            }
            Message::IncrementTenPressed => {
                self.value += 10;
                self.toasts.push(
                    secondary("Success in adding 10")
                        .body(format!("The value is now {}", self.value).as_str()),
                );
            }
            Message::DecrementPressed => {
                self.value -= 1;

                self.toasts.push(success("Removed 1"));
            }
            Message::DecrementTenPressed => {
                self.value += 10;
                self.toasts.push(
                    danger("Removed 10").body(format!("The value is now {}", self.value).as_str()),
                );
            }
            Message::Close(index) => {
                self.toasts.remove(index);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
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
        ]
        .spacing(20);

        Manager::new(content, &self.toasts, Message::Close)
            .timeout(5) // The alert will exist in 5 seconds
            .into()
    }
}

fn main() {
    iced::application(Counter::default, Counter::update, Counter::view)
        .title("Iced-box toasts")
        .run()
        .unwrap();
}
