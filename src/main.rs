extern crate iced;
extern crate rodio;

mod player;
mod style;

use iced::{Column, Container, Element, Length, Sandbox, Settings, scrollable, Scrollable};
use player::*;

pub fn main() {
    Miso::run(Settings {
        window: iced::window::Settings {
            resizable: true,
            decorations: true,
            size: (400, 600),
        },
        ..Settings::default()
    });
}

#[derive(Debug, Clone)]
enum Message {
    PlayerMessage(usize, PlayerMessage),
}

struct Miso {
    scroll: scrollable::State,
    players: Vec<Player>,
}

impl Sandbox for Miso {
    type Message = Message;

    fn new() -> Miso {
        Miso {
            scroll: scrollable::State::new(),
            players: vec![
                Player::new("resources/rain.wav".to_string(), "Rain".to_string()),
                Player::new(
                    "resources/thunderstorm.wav".to_string(),
                    "Thunderstorm".to_string(),
                ),
                Player::new("resources/wind.wav".to_string(), "Forest Wind".to_string()),
                Player::new("resources/forest.wav".to_string(), "Forest".to_string()),
                Player::new("resources/leaves.wav".to_string(), "Leaves".to_string()),
                Player::new("resources/seaside.wav".to_string(), "Seaside".to_string()),
                Player::new("resources/water.wav".to_string(), "Water".to_string()),
                Player::new("resources/bonfire.wav".to_string(), "Bonfire".to_string()),
                Player::new(
                    "resources/summernight.wav".to_string(),
                    "Summer Night".to_string(),
                ),
                Player::new(
                    "resources/coffeeshop.wav".to_string(),
                    "Coffee Shop".to_string(),
                ),
                Player::new("resources/train.wav".to_string(), "Train".to_string()),
                Player::new("resources/fan.wav".to_string(), "Fan".to_string()),
                Player::new(
                    "resources/whitenoise.wav".to_string(),
                    "White Noise".to_string(),
                ),
                Player::new(
                    "resources/pinknoise.wav".to_string(),
                    "Pink Noise".to_string(),
                ),
                Player::new("resources/airplane.wav".to_string(), "Airplane".to_string()),
                Player::new("resources/bubbles.wav".to_string(), "Bubbles".to_string()),
                Player::new(
                    "resources/brownnoise.wav".to_string(),
                    "Brown Noise".to_string(),
                ),
                Player::new(
                    "resources/waterfall.wav".to_string(),
                    "Waterfall".to_string(),
                ),
                Player::new(
                    "resources/tropicalforest.wav".to_string(),
                    "Tropical Forest".to_string(),
                ),
                Player::new("resources/cicadas.wav".to_string(), "Cicadas".to_string()),
                Player::new(
                    "resources/fireplace.wav".to_string(),
                    "Fireplace".to_string(),
                ),
                Player::new(
                    "resources/oceanwaves.wav".to_string(),
                    "Ocean Waves".to_string(),
                ),
                Player::new(
                    "resources/rainontent.wav".to_string(),
                    "Rain on Tent".to_string(),
                ),
                Player::new(
                    "resources/underwater.wav".to_string(),
                    "Underwater".to_string(),
                ),
                Player::new(
                    "resources/spaceengine.wav".to_string(),
                    "Space Engine".to_string(),
                ),
                Player::new(
                    "resources/washingmachine.wav".to_string(),
                    "Washing Machine".to_string(),
                ),
                Player::new(
                    "resources/cityscape.wav".to_string(),
                    "City Scape".to_string(),
                ),
            ],
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
        let Miso {
            scroll,
            ..
        } = self;

        let players =
            self.players
                .iter_mut()
                .enumerate()
                .fold(Column::new(), |col, (i, player)| {
                    col.push(
                        player
                            .view()
                            .map(move |message| Message::PlayerMessage(i, message)),
                    )
                });

        let content = Column::new()
            .spacing(10)
            .padding(20)
            .max_width(600)
            .push(players);


        let scrollable = Scrollable::new(scroll)
            .push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Container)
            .into()
    }
}
