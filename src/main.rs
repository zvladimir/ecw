#![windows_subsystem = "windows"]
use iced::widget::{button, container::Style, row, Column, Container, Text};
use iced::{Color, Element, Fill, Settings, Size, Theme};

mod help;
mod ohm_law;
mod parser;
mod types;
mod voltage_divider;

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .window(iced::window::Settings {
            size: Size {
                width: 800.0,
                height: 600.0,
            },
            min_size: Some(Size {
                width: 800.0,
                height: 600.0,
            }),
            ..Default::default()
        })
        .settings(Settings {
            default_font: iced::Font::DEFAULT,
            ..Default::default()
        })
        .centered()
        .run()
}

#[derive(Default)]
struct App {
    scene: Scene,
}

#[derive(Debug, Clone)]
enum Message {
    SwitchScene(SceneType),
    OhmLawMsg(ohm_law::Message),
    VoltageDivider(voltage_divider::Message),
    Help(help::Message),
}

#[derive(Debug)]
enum Scene {
    OhmLawMsg(ohm_law::OhmLaw),
    VoltageDivider(voltage_divider::VoltageDivider),
    Help(help::Help),
}

#[derive(Debug, Clone)]
enum SceneType {
    OhmLaw,
    VoltageDivider,
    Help,
}

impl Default for Scene {
    fn default() -> Self {
        Scene::OhmLawMsg(ohm_law::OhmLaw::default())
    }
}

impl App {
    fn title(&self) -> String {
        const TITLE_MAIN: &str = "Electrical Calculation Wizard";

        let title_scene = match &self.scene {
            Scene::OhmLawMsg(s) => s.title(),
            Scene::VoltageDivider(s) => s.title(),
            Scene::Help(s) => s.title(),
        };

        format!("{} - {}", title_scene, TITLE_MAIN)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SwitchScene(scene_type) => {
                self.scene = match scene_type {
                    SceneType::OhmLaw => Scene::OhmLawMsg(ohm_law::OhmLaw::default()),
                    SceneType::VoltageDivider => {
                        Scene::VoltageDivider(voltage_divider::VoltageDivider::default())
                    }
                    SceneType::Help => Scene::Help(help::Help::new()),
                };
            }
            Message::VoltageDivider(msg) => {
                if let Scene::VoltageDivider(scene) = &mut self.scene {
                    scene.update(msg);
                }
            }
            Message::OhmLawMsg(msg) => {
                if let Scene::OhmLawMsg(scene) = &mut self.scene {
                    scene.update(msg);
                }
            }
            Message::Help(msg) => {
                if let Scene::Help(scene) = &mut self.scene {
                    scene.update(msg);
                }
            }
        }
    }

    fn view_sidebar(&self) -> Element<Message> {
        Column::new()
            .push(
                button("Ohm Law")
                    .on_press(Message::SwitchScene(SceneType::OhmLaw))
                    .width(Fill),
            )
            .push(
                button("Voltage Divider")
                    .on_press(Message::SwitchScene(SceneType::VoltageDivider))
                    .width(Fill),
            )
            .push(Text::new("").height(Fill))
            .push(
                button("Help")
                    .on_press(Message::SwitchScene(SceneType::Help))
                    .width(Fill),
            )
            .spacing(5)
            .into()
    }

    fn view_context(&self) -> Element<Message> {
        match &self.scene {
            Scene::OhmLawMsg(scene) => scene.view().map(Message::OhmLawMsg),
            Scene::VoltageDivider(scene) => scene.view().map(Message::VoltageDivider),
            Scene::Help(scene) => scene.view().map(Message::Help),
        }
    }

    fn view(&self) -> Element<Message> {
        let sidebar = Container::new(self.view_sidebar())
            .padding(5)
            .width(150)
            .height(Fill)
            .style(|_t: &Theme| Style {
                background: Some(Color::from_rgb8(8, 21, 40).into()),
                ..Style::default()
            });
        let content = Container::new(self.view_context())
            .padding(10)
            .height(Fill)
            .width(Fill)
            .style(|_t: &Theme| Style {
                background: Some(Color::from_rgb8(246, 246, 246).into()),
                ..Style::default()
            });

        row![sidebar, content].into()
    }
}
