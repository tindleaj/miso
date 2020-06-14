use crate::style;

use iced::{slider, Checkbox, Element, Row, Slider};
use rodio::{Decoder, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{channel, Sender};
use std::thread;

#[derive(Debug, Clone)]
pub enum PlayerMessage {
    ChangeVolume(f32),
    Play,
    Pause,
    #[allow(dead_code)]
    Stop,
}

pub struct Player {
    worker: Option<Sender<PlayerMessage>>,
    sound_path: String,
    is_playing: bool,
    volume_slider: slider::State,
    volume: f32,
    label: String,
}

impl Player {
    pub fn new(path: String, label: String) -> Self {
        let player = Player {
            worker: None,
            sound_path: path,
            is_playing: false,
            volume_slider: slider::State::new(),
            volume: 1.0,
            label,
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

            // player
            loop {
                use PlayerMessage::*;

                let msg = rx.recv();
                match msg.unwrap() {
                    ChangeVolume(val) => sink.set_volume(val),
                    Play => sink.play(),
                    Pause => sink.pause(),
                    Stop => break,
                }
            }
        });

        self.worker = Some(tx);
        self
    }

    pub fn update(&mut self, message: PlayerMessage) {
        match &self.worker {
            Some(worker) => {
                match message {
                    PlayerMessage::Play => self.is_playing = true,
                    PlayerMessage::Pause => self.is_playing = false,
                    PlayerMessage::Stop => self.is_playing = false,
                    PlayerMessage::ChangeVolume(vol) => self.volume = vol,
                }

                worker.send(message).unwrap()
            }
            None => todo!("handle?"),
        }
    }

    pub fn view(&mut self) -> Element<PlayerMessage> {
        let checkbox = Checkbox::new(self.is_playing, &self.label, |state| match state {
            true => PlayerMessage::Play,
            false => PlayerMessage::Pause,
        })
        .width(iced::Length::FillPortion(3));

        let slider = Slider::new(&mut self.volume_slider, 0.0..=1.0, self.volume, |state| {
            PlayerMessage::ChangeVolume(state)
        })
        .width(iced::Length::FillPortion(3))
        .style(style::Slider);

        Row::new()
            .push(checkbox)
            .push(slider)
            .padding(5)
            .align_items(iced::Align::Center)
            .spacing(5)
            .into()
    }
}
