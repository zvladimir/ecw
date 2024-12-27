use iced::widget::{markdown, Scrollable};
use iced::{Element, Theme};

use crate::ohm_law;
use crate::voltage_divider;

#[derive(Debug, Clone)]
pub struct Help {
    markdown: Vec<markdown::Item>,
}

#[derive(Debug, Clone)]
pub enum Message {
    LinkClicked(()),
}

impl Help {
    pub fn new() -> Self {
        let help1 = ohm_law::help();
        let help2 = voltage_divider::help();

        let mut t = String::from("# Help\n");
        t.push_str(&format!("## {}\n", &help1.0));
        t.push_str(&help1.1);
        t.push_str("\n\n");
        t.push_str(&format!("## {}\n", &help2.0));
        t.push_str(&help2.1);

        Self {
            markdown: markdown::parse(&t).collect(),
        }
    }

    pub fn title(&self) -> String {
        String::from("Help")
    }

    pub fn view(&self) -> Element<Message> {
        let t = markdown::view(
            &self.markdown,
            markdown::Settings::default(),
            markdown::Style::from_palette(Theme::TokyoNightStorm.palette()),
        )
        .map(|_v| Message::LinkClicked(()));

        Scrollable::new(t).height(iced::Fill).into()
    }

    pub fn update(&mut self, _message: Message) {}
}
