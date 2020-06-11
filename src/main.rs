extern crate iced;
extern crate rodio;

use iced::{Checkbox, Column, Container, Element, Length, Sandbox, Settings};
use rodio::{Decoder, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub fn main() {
    Miso::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    BirdsToggled(bool),
    OtherToggled(bool),
}

struct Miso {
    bird_worker: Option<Sender<bool>>,
    other_worker: Option<Sender<bool>>,
}

impl Sandbox for Miso {
    type Message = Message;

    fn new() -> Miso {
        Miso {
            bird_worker: None,
            other_worker: None,
        }
    }

    fn title(&self) -> String {
        String::from("Miso")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::BirdsToggled(val) => {
                if val == true {
                    let device = rodio::default_output_device().unwrap();
                    let sink = Sink::new(&device);
                    let file = File::open("birds.wav").unwrap();
                    let source = Decoder::new(BufReader::new(file)).unwrap();

                    let (tx, rx) = channel();
                    thread::spawn(move || {
                        sink.append(source.repeat_infinite());

                        loop {
                            let msg = rx.recv().unwrap();
                            if msg == false {
                                break;
                            }
                        }
                    });
                    self.bird_worker = Some(tx);
                } else {
                    match &self.bird_worker {
                        Some(sender) => {
                            let res = sender.send(false);
                            self.bird_worker = None;
                        }
                        None => unimplemented!(),
                    }
                }
            }
            Message::OtherToggled(val) => {
                if val == true {
                    let device = rodio::default_output_device().unwrap();
                    let sink = Sink::new(&device);
                    let file = File::open("waves.wav").unwrap();
                    let source = Decoder::new(BufReader::new(file)).unwrap();

                    let (tx, rx) = channel();
                    thread::spawn(move || {
                        sink.append(source.repeat_infinite());

                        loop {
                            let msg = rx.recv().unwrap();
                            if msg == false {
                                break;
                            }
                        }
                    });
                    self.other_worker = Some(tx);
                } else {
                    match &self.other_worker {
                        Some(sender) => {
                            let res = sender.send(false);
                            self.other_worker = None;
                        }
                        None => unimplemented!(),
                    }
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let checkbox = Checkbox::new(self.bird_worker.is_some(), "Birds", |state| {
            Message::BirdsToggled(state)
        });

        let other_checkbox = Checkbox::new(self.other_worker.is_some(), "Waves", |state| {
            Message::OtherToggled(state)
        });

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(checkbox)
            .push(other_checkbox);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
