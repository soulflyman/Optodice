mod optolith;
use crate::optolith::optolith::*;
mod config;
use config::Config;
mod test_result;
use test_result::TestResult;
use discord_webhook::{DiscordWebHook, Embed};
use gio::prelude::*;
use glib::{Cast, IsA, Object};
use gtk::{Application, Bin, ButtonsType, Container, Dialog, DialogFlags, MessageDialog, MessageType, ResponseType, Widget, prelude::*};
use json::JsonValue;
use std::{cell::RefCell, env, fs, rc::Rc};
mod ability_check_factory;
mod ability_check;
use crate::ability_check_factory::AbilityCheckFactory;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

const COLOR_SUCCESS: u32 = 65280;
const COLOR_FAILURE: u32 = 16711680;


fn main() {
    //debug GTK ui: GTK_DEBUG=interactive cargo run
    let conf: Rc<RefCell<Config>> = Rc::new(RefCell::new(Config::load()));
    
    let path = "./src/talents.json";
    let json_data = fs::read_to_string(path).expect("Unable to read file");
    let talents: JsonValue = json::parse(&json_data).expect("Error: Parsing of json data failed.");

    let heroes: Rc<RefCell<OptolithHeroes>> = Rc::new(RefCell::new(OptolithHeroes::new()));

    let app = Application::new(
        Some("net.farting-unicorn.optodice"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Failed to initialize GTK.");
  
    app.connect_activate(clone!(conf,heroes => move |app| {
        if !conf.borrow().is_webhook_url_set() {
            request_webhook_url_from_user(&mut conf.borrow_mut());
        }

        let box_main = gtk::Box::new(gtk::Orientation::Vertical, 10);

        let cbt_hero_select = build_hero_select(heroes.borrow().get_simple_hero_list(), conf.borrow().get_last_used_hero_id());
        
        cbt_hero_select.connect_changed(clone!(conf => move |hero_select| {            
            let hero_id = hero_select.get_active_id().expect("Unknown hero selected, this should not happen.");            
            conf.borrow_mut().set_last_used_hero_id(hero_id.to_string());
        }));

        box_main.add(&cbt_hero_select);

        let nb_talents = gtk::Notebook::new();
        box_main.add(&nb_talents);

        for (talent_category, talent_sub) in talents.entries() {
            let lbo_talents = gtk::ListBox::new();
            lbo_talents.set_selection_mode(gtk::SelectionMode::None);

            let nb_tab_natur = gtk::Label::new(Some(talent_category));
            nb_talents.append_page(&lbo_talents, Some(&nb_tab_natur));

            for (talent_id, talent) in talent_sub.entries() {
                let box_talent = gtk::Box::new(gtk::Orientation::Horizontal, 0);

                let lbl_talent_name = build_talent_name_label(&talent_id, &talent);
                box_talent.add(&lbl_talent_name);
                box_talent.set_child_packing(&lbl_talent_name, true, true, 0, gtk::PackType::Start);

                let lbl_test = build_test_label(&talent);
                box_talent.add(&lbl_test);

                /*
                let lbl_talent_value = gtk::Label::new(Some("-"));
                lbl_talent_value.set_halign(gtk::Align::End);
                lbl_talent_value.set_justify(gtk::Justification::Right);
                lbl_talent_value.set_property_width_request(30);
                box_talent.add(&lbl_talent_value);
                */

                let en_talent_test_difculty = build_difficulty_entry(&talent_id);
                box_talent.add(&en_talent_test_difculty);

                let btn_die = build_test_button(&conf, &heroes.borrow_mut(), &talent_id);
                box_talent.add(&btn_die);

                lbo_talents.add(&box_talent);
            }
        }

        //let window: gtk::Window = builder.get_object("window").unwrap();
        let window_builder = gtk::WindowBuilder::new();
        let window = window_builder.build();
        window.set_title("Optodice");
        window.add(&box_main);
        window.set_application(Some(app));
        window.show_all();
    }));

    app.run(&env::args().collect::<Vec<_>>());
}

fn fire_webhook(conf: &Config, heroes: OptolithHeroes, die_result: TestResult) {
    let mut embed = Embed::default();
    embed.description = Some(die_result.get_formated());
    if die_result.is_success() {
        embed.color = Some(COLOR_SUCCESS);
    } else {
        embed.color = Some(COLOR_FAILURE);
    }
    
    let mut avatar_url = conf.get_avatar_base_url();
    if !avatar_url.ends_with("/") {
        avatar_url.push_str("/");
    }
    avatar_url.push_str(conf.get_last_used_hero_id().as_str());
    avatar_url.push_str(".png");

    let mut webhook = DiscordWebHook::new_with_embed(conf.get_webhook_url().as_str(), embed);
    //let mut webhook = DiscordWebHook::new(conf.get_webhook_url().as_str(), avatar_url.as_str());
    //webhook.add_embed(embed);
    webhook.set_avatar_url(avatar_url.as_str());
    webhook.set_username(heroes.get_hero_name_by_id(conf.get_last_used_hero_id()).as_str());
    webhook.fire();
}

fn request_webhook_url_from_user(conf: &mut Config) {
    let dialog = Dialog::with_buttons::<gtk::Window>(
        Some("No webhook URL was found in the config.toml."),
        None,
        DialogFlags::MODAL,
        &[("lala", ResponseType::Apply)]
    );
    dialog.set_modal(true);

    let dialog_label = gtk::Label::new(Some("Please enter the URL of the Discord Webhook."));
    dialog.get_content_area().add(&dialog_label);   
    
    let webhook_url_entry = gtk::Entry::new();              
    webhook_url_entry.set_activates_default(true);
    dialog.set_default_response(ResponseType::Apply);
    dialog.get_content_area().add(&webhook_url_entry);     
    dialog.show_all();         

    let response_type = dialog.run();
    if response_type == ResponseType::Apply {
        let text = webhook_url_entry.get_text().to_string().trim().to_string();
        if text.is_empty() {
            abort_app();
        }

        conf.set_webhook_url(text);
        return;

    } else {
        abort_app();
    }
    dialog.close();
}

fn abort_app() {
    let msg_dialog = MessageDialog::new::<gtk::Window>(
        None,
        DialogFlags::MODAL,
        MessageType::Error,
        ButtonsType::Ok,
        "No hook, no game.",
    );
    msg_dialog.set_title("We need more hooks!");
    msg_dialog.connect_response(|_, _| std::process::exit(1));
    msg_dialog.run();
}

fn build_talent_name_label(talent_id: &str, talent: &JsonValue) -> gtk::Label {
    let lbl_talent_name = gtk::Label::new(talent["name"].as_str());
    lbl_talent_name.set_widget_name(format!("{}#label", talent_id).as_str());
    lbl_talent_name.set_halign(gtk::Align::Start);
    lbl_talent_name
}

fn build_test_button(conf: &Rc<RefCell<Config>>, heroes: &OptolithHeroes, talent_id: &str) -> gtk::Button {
    let btn_die = gtk::Button::with_label("🎲");
    btn_die.set_widget_name(format!("{}#button", talent_id).as_str());
    let local_heroes = heroes.clone();
    let local_conf = conf.clone();
    btn_die.connect_clicked(clone!(conf => move |but| {
        let hero_id = get_hero_id(&but);
        let talent_id = get_talent_id(&but);
        let difficulty = get_test_difficulty(&but);
        role_test(&conf.borrow_mut(), local_heroes.clone(), &hero_id, &talent_id, difficulty);
    }));
    return btn_die;
}

fn build_test_label(talent: &JsonValue) -> gtk::Label {
    let mut talent_test_label_text = String::default();
    if talent["test"].len() == 3 {
        talent_test_label_text = format!(
            "{} / {} / {}",
            talent["test"][0], talent["test"][1], talent["test"][2]
        );
    }
    let lbl_talent_test = gtk::Label::new(Some(talent_test_label_text.as_str()));
    lbl_talent_test.set_justify(gtk::Justification::Right);
    lbl_talent_test.set_property_width_request(100);
    lbl_talent_test
}

fn build_difficulty_entry(talent_id: &str) -> gtk::Entry {
    let en_talent_test_difculty = gtk::Entry::new();
    en_talent_test_difculty.set_widget_name(format!("{}#difficulty", talent_id).as_str());
    en_talent_test_difculty.set_alignment(0.5);
    en_talent_test_difculty.set_placeholder_text(Some("+/-"));
    en_talent_test_difculty.set_width_chars(4);
    en_talent_test_difculty.set_max_length(4);
    en_talent_test_difculty
}

fn build_hero_select(hero_list: Vec<SimpleHero>, last_used_hero_id: String) -> gtk::ComboBoxText {
    if hero_list.len() == 0 {
        let dialog = MessageDialog::new::<gtk::Window>(
            None,
            DialogFlags::MODAL,
            MessageType::Error,
            ButtonsType::Ok,
            "No heroes found in heroes.json.",
        );
        dialog.set_title("We need more hereos!");
        dialog.connect_response(|_, _| std::process::exit(1));
        dialog.run();
    }
    let hero_select = gtk::ComboBoxText::new();
    for hero in hero_list.clone() {
        hero_select.append(Some(hero.id.as_str()), hero.name.as_str());
    }
    
    hero_select.set_widget_name("hero_select");
    if last_used_hero_id.is_empty() {
        hero_select.set_active(Some(0));
    } else {
        hero_select.set_active_id(Some(last_used_hero_id.as_str()));
    }
    
    return hero_select;
}

fn get_hero_id(button: &gtk::Button) -> String {
    //todo better handling of this ugly expect chain
    let box_main = button
        .get_parent()
        .expect("fuck1")
        .get_parent()
        .expect("fuck2")
        .get_parent()
        .expect("fuck3")
        .get_parent()
        .expect("fuck4")
        .get_parent()
        .expect("fuck5");
    let widget_search_name = "hero_select";
    let hero_select: gtk::ComboBoxText =
        find_child_by_name(&box_main, widget_search_name).expect("Error: Failed to find child");
    return hero_select
        .get_active_id()
        .expect("Error: No hero selected.")
        .to_string();
}

fn get_talent_id(button: &gtk::Button) -> String {
    let btn_name = button.get_widget_name();
    let btn_name_split = btn_name.as_str().split('#').collect::<Vec<_>>();
    return btn_name_split[0].to_string();
}

fn get_test_difficulty(button: &gtk::Button) -> i32 {
    let talent_id = get_talent_id(button);
    let parent_widget = button
        .get_parent()
        .expect("Error: Failed to get parent widget of pressed button.");
    let widget_search_name = format!("{}#difficulty", talent_id);
    let talent_label: gtk::Entry = find_child_by_name(&parent_widget, widget_search_name.as_str())
        .expect("Error: Failed to find child");
    return talent_label
        .get_text()
        .to_string()
        .parse::<i32>()
        .or::<i32>(Ok(0))
        .unwrap();
}

fn role_test(conf: &Config, heroes: OptolithHeroes, hero_id: &String, skill_id: &String, difficulty: i32) {
    let tv = heroes.get_skill_value(hero_id, skill_id);
    println!(
        "Hero {} roles test for {} (TV {}) with difficulty {}",
        hero_id, skill_id, tv, difficulty
    );

    let mut factory = AbilityCheckFactory::new(heroes.clone());
    let mut ability_check = factory.get_ability_check(hero_id.to_owned(), skill_id.to_owned());
    let check_result = ability_check.check_ability(&difficulty);
   
    /*
    let die_result = TestResult {
        ability_name: "Rambolen".to_string(),
        ability_score: 0,
        skill_names: vec!("KK".to_string(), "KL".to_string(), "KO".to_string()),
        skill_values: vec!(8,12,11),
        dice_values: vec!(6,1,10),
        difficulty: 0,
        quality: 2,
        success: true,        
    };*/
    fire_webhook(&conf, heroes, check_result);
}

/// Returns the child element which has the given name.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{prelude::BuildableExtManual, Button, ContainerExt, WidgetExt, Window, WindowType};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let but = Button::new();
/// let w = Window::new(WindowType::Toplevel);
///
/// but.set_widget_name("Button");
/// w.add(&but);
///
/// gtk_test::find_child_by_name::<Button, Window>(&w, "Button").expect("failed to find child");
/// // Or even better:
/// let but: Button = gtk_test::find_child_by_name(&w, "Button").expect("failed to find child");
/// # }
/// ```
pub fn find_child_by_name<C: IsA<Widget>, W: Clone + IsA<Object> + IsA<Widget>>(
    parent: &W,
    name: &str,
) -> Option<C> {
    find_widget_by_name(parent, name).and_then(|widget| widget.downcast().ok())
}

/// Returns the child widget which has the given name.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{Button, ContainerExt, WidgetExt, Window, WindowType};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let but = Button::new();
/// let w = Window::new(WindowType::Toplevel);
///
/// but.set_widget_name("Button");
/// w.add(&but);
///
/// gtk_test::find_widget_by_name(&w, "Button").unwrap();
/// # }
/// ```
pub fn find_widget_by_name<W: Clone + IsA<Object> + IsA<Widget>>(
    parent: &W,
    name: &str,
) -> Option<Widget> {
    if let Ok(container) = parent.clone().dynamic_cast::<Container>() {
        for child in container.get_children() {
            if child.get_widget_name() == name {
                return Some(child);
            }
            if let Some(widget) = find_widget_by_name(&child, name) {
                return Some(widget);
            }
        }
    } else if let Ok(bin) = parent.clone().dynamic_cast::<Bin>() {
        if let Some(child) = bin.get_child() {
            if child.get_widget_name() == name {
                return Some(child);
            }
            if let Some(widget) = find_widget_by_name(&child, name) {
                return Some(widget);
            }
        }
    }
    None
}
