pub mod ui_hero_select {
    use iced::{Align, Button, button, Column, Element, Sandbox, Settings, Text, PickList, pick_list, Command};

    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    struct Hero {
        pub index: u32,
        pub name: String
    }

    impl std::fmt::Display for Hero {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.name)
        }
    }

    #[derive(Default)]
    pub struct HeroSelect {
        heroes: Vec<Hero>,
        hero_list: pick_list::State<Hero>,
        selected_hero: Hero,
    }

    #[derive(Debug, Clone)]
    pub enum Message {
        HeroSelected(Hero),
    }

    impl Sandbox for HeroSelect {
        type Message = Message;
        
        fn new() -> Self {
            let mut bla = Self::default();
            bla.heroes = vec![Hero{index:1, name: String::from("held 1")}, Hero{index:2, name: String::from("held 2")}, Hero{index:3, name: String::from("held 3")}];
            bla
        }

        fn title(&self) -> String {
            String::from("Optodice - HeroSelect")            
        }

        fn update(&mut self, message: Message) {
            match message {
                Message::HeroSelected(hero) => {
                    self.selected_hero = hero.clone();
                    println!("Message selected: {:?}", hero.clone());
                }
            }
        }

        fn view(&mut self) -> Element<Message> {
            Column::new()
                .padding(20)
                .align_items(Align::Center)
                .push(
                    PickList::new( &mut self.hero_list, &self.heroes, Some(self.selected_hero.clone()), Message::HeroSelected)
                )
                .into()
        }
    }
}