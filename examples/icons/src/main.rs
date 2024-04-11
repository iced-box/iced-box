use iced::{font, Font, Command, Element, Application, Settings};
use iced::widget::{button, column, text};
use iced_box::icon::{
    Lucide,
    lucide_font,
    load_lucide_font,
    Material,
    material_font,
    load_material_font,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    FontLoaded(Result<(), font::Error>),
}

#[derive(Debug)]
struct Counter {
    // The counter value
    value: i32,
}

impl Application for Counter {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();
    type Theme = iced::theme::Theme;
    
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                value: 0,
            },
            Command::batch(vec![
                load_material_font().map(Message::FontLoaded),
                load_lucide_font().map(Message::FontLoaded),
                Command::none(),
            ])
        )
    }
    
    fn title(&self) -> String {
        "Icex-box icons".to_string()
    }
    
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
                text(Material::PlusOne).font(material_font)
            ).on_press(Message::IncrementPressed),

            // We show the value of the counter here
            text(self.value).size(50),

            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button(
                text(Lucide::Minus).font(lucide_font)
            ).on_press(Message::DecrementPressed),
            text(Lucide::Plane).font(lucide_font)
        ].into()
    }
}
fn main() {
    let _ = Counter::run(Settings::default());
}
