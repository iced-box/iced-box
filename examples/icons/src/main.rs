extern crate iced;
extern crate iced_box;

use iced::{Font, Command, Element};
use iced::widget::{button, column, text};
use iced_box::icon::{
    LoadingResult,
    lucide::*,
    material::*,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    FontLoaded(LoadingResult),
}

#[derive(Debug, Default)]
struct Counter {
    // The counter value
    value: i32,
}

impl Counter {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            },
            _ => {},
        }

        Command::none()
    }
    
    fn view(&self) -> Element<Message> {
        let lucide_font : Font = lucide_font();
        let material_font : Font = material_font();

        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `IncrementPressed` message when pressed
            button(
                text(Material::PlusOne.to_string()).font(material_font)
            ).on_press(Message::IncrementPressed),

            // We show the value of the counter here
            text(self.value).size(50),

            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button(
                text(Lucide::Minus.to_string()).font(lucide_font)
            ).on_press(Message::DecrementPressed),
            text(Lucide::Plane.to_string()).font(lucide_font)
        ].into()
    }
}
fn main() {
    let _ = iced::program(
        "Icex-box icons",
        Counter::update,
        Counter::view
    ).font(
        load_material_font()
    ).font(
        load_lucide_font()
    ).run();
}
