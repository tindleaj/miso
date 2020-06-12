extern crate iced;
extern crate rodio;

use iced::{Checkbox, Row,  Column, Container, Element, Length, Sandbox, Settings};
use rodio::{Decoder, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub fn main() {
    Miso::run(Settings::default())
}

#[derive(Debug, Clone)]
enum PlayerMessage {
    ChangeVolume(f32),
    Play,
    Pause,
    Stop
}

struct Player {
    worker: Option<Sender<PlayerMessage>>,
    sound_path: String,
    is_playing: bool,
    label: String
}

impl Player {
    pub fn new(path: String, label: String) -> Self {
        let player = Player {
            worker: None,
            sound_path: path,
            is_playing: false,
            label
        };

        player.start()
    }

    fn start(mut self) -> Self {
        let device = rodio::default_output_device().unwrap();
        let sink = Sink::new(&device);
        let file = File::open(&self.sound_path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();

        let (tx, rx) = channel();

        thread::spawn(move || {
            sink.append(source.repeat_infinite());
            sink.pause();

            'player: loop {
                use PlayerMessage::*;

                let msg = rx.recv();
                match msg.unwrap() {
                    ChangeVolume(val) => sink.set_volume(val),
                    Play => sink.play(),
                    Pause => sink.pause(),
                    Stop => break
                }
            }

        });

        self.worker = Some(tx);
        self
    }

    fn update(&mut self, message: PlayerMessage) {
        match &self.worker {
            Some(worker) => {
                match message {
                    PlayerMessage::Play => self.is_playing = true,
                    PlayerMessage::Pause => self.is_playing = false,
                    PlayerMessage::Stop => self.is_playing = false,
                    _ => unimplemented!()
                }

                worker.send(message).unwrap()
            } ,
            None => todo!("handle?")
        }
        
    }

    fn view(&mut self) -> Element<PlayerMessage> {
        let checkbox = Checkbox::new(self.is_playing, &self.label, |state| {
            match state {
                true => PlayerMessage::Play,
                false => PlayerMessage::Pause
            }
        });

        Row::new().push(checkbox).into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    PlayerMessage(usize, PlayerMessage)
}

struct Miso {
    players: Vec<Player>
}

impl Sandbox for Miso {
    type Message = Message;

    fn new() -> Miso {
        Miso {
            players: vec![
                Player::new("sounds/birds.wav".to_string(), "Birds".to_string()),
                Player::new("sounds/waves.wav".to_string(), "Waves".to_string())
            ]
        }
    }

    fn title(&self) -> String {
        String::from("Miso")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PlayerMessage(i, message) => {
                if let Some(player) = self.players.get_mut(i) {
                    player.update(message);
                }
            } 
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let players = self.players
            .iter_mut()
            .enumerate()
            .fold(Column::new(), |col, (i, player)| {
                col.push(player.view().map(move |message| {
                    Message::PlayerMessage(i, message)
                }))
        });

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(players);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
