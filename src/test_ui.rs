pub mod test_ui {
    use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text, PickList, pick_list};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Language {
        Rust,
        Elm,
        Ruby,
        Haskell,
        C,
        Javascript,
        Other,
    }
    impl Language {
        const ALL: [Language; 7] = [
            Language::C,
            Language::Elm,
            Language::Ruby,
            Language::Haskell,
            Language::Rust,
            Language::Javascript,
            Language::Other,
        ];
    }
    
    impl Default for Language {
        fn default() -> Language {
            Language::Rust
        }
    }

    impl std::fmt::Display for Language {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Language::Rust => "Rust",
                    Language::Elm => "Elm",
                    Language::Ruby => "Ruby",
                    Language::Haskell => "Haskell",
                    Language::C => "C",
                    Language::Javascript => "Javascript",
                    Language::Other => "Some other language",
                }
            )
        }
    }

    #[derive(Default)]
    pub struct Counter {
        value: i32,
        increment_button: button::State,
        decrement_button: button::State,
        pick_list: pick_list::State<Language>,
        selected_language: Language,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Message {
        IncrementPressed,
        DecrementPressed,
        LanguageSelected(Language),
    }

    impl Sandbox for Counter {
        type Message = Message;

        fn new() -> Self {
            Self::default()
        }

        fn title(&self) -> String {
            String::from("Counter - Iced")
        }

        fn update(&mut self, message: Message) {
            match message {
                Message::IncrementPressed => {
                    self.value += 1;
                }
                Message::DecrementPressed => {
                    self.value -= 1;
                }
                Message::LanguageSelected(language) => {
                    self.selected_language = language;
                    println!("Message selected: {}", language);
                }
            }
        }

        fn view(&mut self) -> Element<Message> {
            Column::new()
                .padding(20)
                .align_items(Align::Center)
                .push(
                    Button::new(&mut self.increment_button, Text::new("Increment"))
                        .on_press(Message::IncrementPressed),
                )
                .push(Text::new(self.value.to_string()).size(50))
                .push(
                    Button::new(&mut self.decrement_button, Text::new("Decrement"))
                        .on_press(Message::DecrementPressed),
                )
                .push(
                    PickList::new( &mut self.pick_list, &Language::ALL[..], Some(self.selected_language), Message::LanguageSelected)
                )
                .into()
        }
    }
}