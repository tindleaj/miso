extern crate iced;
extern crate rodio;

use iced::{Checkbox, Column, Container, Element, Length, Sandbox, Settings};
use std::io::BufReader;
use std::thread::JoinHandle;
use std::time::Duration;

// fn main() {
//     let device = rodio::default_output_device().unwrap();

//     let file = std::fs::File::open("birds.wav").unwrap();
//     let play = rodio::play_once(&device, BufReader::new(file)).unwrap();
//     play.set_volume(1.0);
//     println!("Started play");

//     thread::sleep(Duration::from_millis(15000));
// }

fn play_audio() -> JoinHandle<()> {
    std::thread::spawn(|| loop {
        println!("Started play");
        let device = rodio::default_output_device().unwrap();
        let file = std::fs::File::open("birds.wav").unwrap();
        let play = rodio::play_once(&device, BufReader::new(file)).unwrap();
        play.set_volume(1.0);
        play.sleep_until_end();
    })
}

pub fn main() {
    Miso::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    SoundToggled(bool),
}

#[derive(Default)]
struct Miso {
    toggle: bool,
}

impl Sandbox for Miso {
    type Message = Message;

    fn new() -> Miso {
        Miso::default()
    }

    fn title(&self) -> String {
        String::from("Miph")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SoundToggled(true) => {
                dbg!("play");
                self.toggle = true;
                play_audio();
            }
            Message::SoundToggled(false) => {
                dbg!("pause");
                self.toggle = false
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let checkbox = Checkbox::new(self.toggle, "Toggle", Message::SoundToggled);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(checkbox);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
