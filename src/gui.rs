use iced::{Application, Command, Element, Settings, Text, executor, button, text_input, Column, Row, Button, TextInput};
use iced::window;
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum Message {
    InputDirChanged(String),
    OutputDirChanged(String),
    KeyChanged(String),
    ExcludePatternChanged(String),
    SelectInputDir,
    SelectOutputDir,
    OperationChanged(Operation),
    Execute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Encrypt,
    Decrypt,
}

struct McrpGUI {
    input_dir: PathBuf,
    output_dir: PathBuf,
    key: String,
    exclude_pattern: String,
    operation: Operation,
    input_dir_btn: button::State,
    output_dir_btn: button::State,
    execute_btn: button::State,
}

impl Application for McrpGUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self {
            input_dir: PathBuf::new(),
            output_dir: PathBuf::new(),
            key: String::new(),
            exclude_pattern: String::new(),
            operation: Operation::Encrypt,
            input_dir_btn: button::State::default(),
            output_dir_btn: button::State::default(),
            execute_btn: button::State::default(),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("MCRP Utility GUI")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputDirChanged(path) => self.input_dir = PathBuf::from(path),
            Message::OutputDirChanged(path) => self.output_dir = PathBuf::from(path),
            Message::KeyChanged(key) => self.key = key,
            Message::ExcludePatternChanged(pattern) => self.exclude_pattern = pattern,
            Message::OperationChanged(op) => self.operation = op,
            Message::SelectInputDir => {
                return Command::perform(
                    iced_native::dialog::FileDialog::new().pick_directory(),
                    |path| Message::InputDirChanged(path.unwrap().to_string_lossy().into_owned())
                );
            }
            Message::SelectOutputDir => {
                return Command::perform(
                    iced_native::dialog::FileDialog::new().pick_directory(),
                    |path| Message::OutputDirChanged(path.unwrap().to_string_lossy().into_owned())
                );
            }
            Message::Execute => {
                let input = self.input_dir.clone();
                let output = self.output_dir.clone();
                let key = self.key.clone();
                
                match self.operation {
                    Operation::Encrypt => {
                        crate::encrypt(&input, &output, key.as_bytes(), vec![self.exclude_pattern.clone()]);
                    }
                    Operation::Decrypt => {
                        if key.len() != 32 {
                            eprintln!("密钥长度必须为32位");
                            return Command::none();
                        }
                        crate::decrypt(&input, &output, key.as_bytes());
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let operation_selector = Row::new()
            .push(iced::PickList::new(
                &[Operation::Encrypt, Operation::Decrypt][..],
                self.operation,
                Message::OperationChanged
            ))
            .push(Button::new(&mut self.execute_btn, Text::new("执行")).on_press(Message::Execute))
            .spacing(20);

        Column::new()
            .push(Text::new("Minecraft资源包加解密工具").size(24))
            .push(
                Row::new()
                    .push(TextInput::new("输入目录", &self.input_dir.to_string_lossy(), Message::InputDirChanged))
                    .push(Button::new(&mut self.input_dir_btn, Text::new("选择")).on_press(Message::SelectInputDir))
            )
            .push(
                Row::new()
                    .push(TextInput::new("输出目录", &self.output_dir.to_string_lossy(), Message::OutputDirChanged))
                    .push(Button::new(&mut self.output_dir_btn, Text::new("选择")).on_press(Message::SelectOutputDir))
            )
            .push(TextInput::new("密钥", &self.key, Message::KeyChanged))
            .push(TextInput::new("排除模式", &self.exclude_pattern, Message::ExcludePatternChanged))
            .push(operation_selector)
            .into()
    }
}

pub fn run() -> iced::Result {
    McrpGUI::run(Settings {
        window: window::Settings {
            size: (800, 600),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}