extern crate font;
use iced::{Button, Element, Sandbox, Settings, Text, button, Column, Align};

pub fn main()-> iced::Result{
    Font::run(Settings::default())
}
#[derive(Debug,Default)]
pub struct Font{
    btn:button::State,
}
#[derive(Debug,Clone, Copy)]
pub enum FontMessage{
    BtnClicked,
}

impl Sandbox for Font {
    type Message = FontMessage;    

    fn new() -> Self {
        Self::default()
        
    }

    fn title(&self) -> String {
        String::from("Font Testing")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            FontMessage::BtnClicked => {
                println!("clicked");
            }
        }
        // This application has no interactions
        
    }

    fn view(&mut self) -> Element<Self::Message> {  
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .spacing(20)
            .push(
                Text::new("Font will change when you click button").size(24)
            )
            .push(
                Button::new(&mut self.btn, Text::new("Click Me"))
                    .on_press(FontMessage::BtnClicked),
            )
            
            .into()
    }
}
