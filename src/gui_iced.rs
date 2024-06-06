use iced::{Application, Theme, Subscription};
use iced::widget::button;
// use iced::window;

pub struct MyApp {
    watch_button: button::State,
    target_button: button::State,
    watch_dir: String,
    target_dir: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    WatchDirSelected(String),
    TargetDirSelected(String),
}

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            MyApp {
                watch_button: button::State::new(),
                target_button: button::State::new(),
                watch_dir: String::new(),
                target_dir: String::new(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::WatchDirSelected(dir) => {
                self.watch_dir = dir;
            }
            Message::TargetDirSelected(dir) => {
                self.target_dir = dir;
            }
        }
        iced::Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::Subscription::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::Column::new()
            .push(
                iced::widget::Button::new("Watch")
                    // .on_press(|| {
                    //     let dialog = iced::widget::native::dialog::FileDialog::new();
                    //     dialog.show_open_single_dir("Select Watch Directory", move |path| {
                    //         if let Some(path) = path {
                    //             Message::WatchDirSelected(path.to_string_lossy().to_string())
                    //         } else {
                    //             Message::WatchDirSelected("".to_string())
                    //         }
                    //     });
                    // }),
            )
            // .push(
            //     iced::widget::Button::new("Target")
            //         .on_press(Message::TargetDirSelected("".to_string()))
            //         .on_release(|| {
            //             let dialog = iced::native::dialog::FileDialog::new();
            //             dialog.show_open_single_dir("Select Target Directory", move |path| {
            //                 if let Some(path) = path {
            //                     Message::TargetDirSelected(path.to_string_lossy().to_string())
            //                 } else {
            //                     Message::TargetDirSelected("".to_string())
            //                 }
            //             });
            //         }),
            // )
            .push(iced::widget::text(format!("Watch Directory: {}", self.watch_dir)))
            // .push(iced::widget::text(format!("Target Directory: {}", self.target_dir)))
            .into()
    }
}
