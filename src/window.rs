use cosmic::{app::Core, widget, Application, Command};

use crate::markdown::markdown;

#[derive(Debug, Clone)]
pub enum Message {}

pub struct Window {
    core: Core,
    content1: String,
    content2: String,
    content3: String,
}

impl Application for Window {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "io.github.elevenhsoft.Markdown";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, cosmic::app::Command<Self::Message>) {
        let content1 = include_str!("../code.rs").to_string();
        let content2 = include_str!("../code2.py").to_string();
        let content3 = include_str!("../markdown.md").to_string();

        (
            Self {
                core,
                content1,
                content2,
                content3,
            },
            Command::none(),
        )
    }

    fn view(&self) -> cosmic::prelude::Element<Self::Message> {
        let code1 = markdown(&self.content1, "rs");
        let code2 = markdown(&self.content2, "py");
        let md3 = markdown(&self.content3, "md");

        let row = widget::row().push(code1).push(code2).push(md3);
        row.into()
    }
}
