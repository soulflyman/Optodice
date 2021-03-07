mod optolith_heroes;
mod config;
mod test_result;
mod skill_check_factory;
mod skill_check;
mod optolith_attributes;
mod optolith_skills;
mod context;

use crate::optolith_heroes::optolith::*;
use config::Config;
use test_result::TestResult;
use discord_webhook::{DiscordWebHook, Embed};
use gio::prelude::*;
use glib::{Cast, IsA, Object};
use gtk::{Application, Bin, ButtonsType, Container, Dialog, DialogFlags, MessageDialog, MessageType, ResponseType, Widget, prelude::*};
use json::JsonValue;
use std::{cell::RefCell, env, error::Error, fs, rc::Rc};
use crate::skill_check_factory::SkillCheckFactory;
use crate::optolith_skills::OptolithSkills;
use crate::optolith_attributes::OptolithAttributes;
use crate::context::Context;

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

#[derive(Debug, Clone)]
pub struct CheckFactories {
    skills: SkillCheckFactory,    
}

const COLOR_SUCCESS: u32 = 65280;
const COLOR_FAILURE: u32 = 16711680;

fn main() {
    //debug GTK ui: GTK_DEBUG=interactive cargo run
    let context = Rc::new(RefCell::new(Context {
        config: Config::load(),
        heroes: OptolithHeroes::new(),
        attributes: OptolithAttributes::new(),
        skills: OptolithSkills::new(),
        active_hero_id: String::default(),
    }));
   
    //TODO use check_factories in button actions!
    //let check_factories = CheckFactories {
    //    skills: SkillCheckFactory::new(&context.borrow()),
    //};    

    let last_used_hero_id = context.borrow().config.get_last_used_hero_id().clone();
    context.borrow_mut().active_hero_id = last_used_hero_id;
    
    let app = Application::new(
        Some("net.farting-unicorn.optodice"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Failed to initialize GTK.");
    
    app.connect_activate(clone!(context => move |app| {
        if !context.borrow().config.is_webhook_url_set() {
            context.borrow_mut().config.set_webhook_url(request_webhook_url_from_user());
        }

        let box_main = gtk::Box::new(gtk::Orientation::Vertical, 10);

        let cbt_hero_select = build_hero_select(&context.borrow());        
        cbt_hero_select.connect_changed(clone!(context => move |hero_select| {            
            let hero_id = hero_select.get_active_id().expect("Unknown hero selected, this should not happen.");            
            context.borrow_mut().config.set_last_used_hero_id(hero_id.to_string());
            context.borrow_mut().active_hero_id = hero_id.to_string();
        }));

        box_main.add(&cbt_hero_select);

        let notebook = gtk::Notebook::new();
        box_main.add(&notebook);

        ui_add_tab_attributes(&notebook, &context.borrow());
        ui_add_tabs_skills(&notebook, &context.borrow());

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

fn ui_add_tabs_skills(notebook: &gtk::Notebook, context: &Context) {
    //TODO get rid of json parsing, skills.json is in context.skills struct
    let path = "./skills.json";
    let json_data = fs::read_to_string(path).expect("Unable to read file");
    let skills_json: JsonValue = json::parse(&json_data).expect("Error: Parsing of json data failed.");
    //let skills = OptolithSkills::new("./skills.json");

    for (skill_category, skill_sub) in skills_json.entries() {
        let lbo_skills = gtk::ListBox::new();
        lbo_skills.set_selection_mode(gtk::SelectionMode::None);

        let nb_tab_name = gtk::Label::new(Some(skill_category));
        notebook.append_page(&lbo_skills, Some(&nb_tab_name));

        for (skill_id, skill) in skill_sub.entries() {
            let box_skill = gtk::Box::new(gtk::Orientation::Horizontal, 0);

            let lbl_skill_name = build_skill_name_label(&skill_id, &skill);
            box_skill.add(&lbl_skill_name);
            box_skill.set_child_packing(&lbl_skill_name, true, true, 0, gtk::PackType::Start);

            let lbl_checks = build_checks_label(&skill_id.to_string(), context);
            box_skill.add(&lbl_checks);
            
            let lbl_skill_value = gtk::Label::new(Some(context.heroes.get_skill_value(&context.active_hero_id, &skill_id.to_string()).to_string().as_str()));
            lbl_skill_value.set_halign(gtk::Align::End);
            lbl_skill_value.set_justify(gtk::Justification::Right);
            lbl_skill_value.set_property_width_request(30);
            box_skill.add(&lbl_skill_value);            

            let en_skill_test_difculty = build_difficulty_entry(&skill_id);
            box_skill.add(&en_skill_test_difculty);

            let btn_die = build_test_button(&context, &skill_id);
            box_skill.add(&btn_die);

            lbo_skills.add(&box_skill);
        }
    }
}

fn ui_add_tab_attributes(notebook: &gtk::Notebook, context: &Context) {
    //TODO get rid of json parsing, attributes.json is in context.attributes struct
    let path = "./attributes.json";
    let json_data = fs::read_to_string(path).expect("Unable to read file");
    let attributes_json: JsonValue = json::parse(&json_data).expect("Error: Parsing of json data failed.");

    let lbo_attributes = gtk::ListBox::new();
    lbo_attributes.set_selection_mode(gtk::SelectionMode::None);

    let nb_tab_name = gtk::Label::new(Some("Attribute"));
    notebook.append_page(&lbo_attributes, Some(&nb_tab_name));

    for (attribute_id, attribute) in attributes_json.entries() {
        let box_attribute = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let lbl_attribute_name = build_skill_name_label(&attribute_id, &attribute);
        box_attribute.add(&lbl_attribute_name);
        box_attribute.set_child_packing(&lbl_attribute_name, true, true, 0, gtk::PackType::Start);
        
        let lbl_attribute_value = gtk::Label::new(Some(context.heroes.get_attribute_value(&context.active_hero_id, &attribute_id.to_string()).to_string().as_str()));
        lbl_attribute_value.set_halign(gtk::Align::End);
        lbl_attribute_value.set_justify(gtk::Justification::Right);
        lbl_attribute_value.set_property_width_request(30);
        box_attribute.add(&lbl_attribute_value);        
        
        let en_atribute_test_difculty = build_difficulty_entry(&attribute_id);
        box_attribute.add(&en_atribute_test_difculty);

        let btn_die = build_test_button(&context, &attribute_id);
        box_attribute.add(&btn_die);

        lbo_attributes.add(&box_attribute);
    }
}

fn fire_webhook(context: &Context, die_result: TestResult) {
    let mut embed = Embed::default();
    embed.description = Some(die_result.get_formated());
    if die_result.is_success() {
        embed.color = Some(COLOR_SUCCESS);
    } else {
        embed.color = Some(COLOR_FAILURE);
    }
    
    let mut avatar_url = context.config.get_avatar_base_url();
    if !avatar_url.ends_with("/") {
        avatar_url.push_str("/");
    }
    avatar_url.push_str(context.active_hero_id.as_str());
    avatar_url.push_str(".png");

    let mut webhook = DiscordWebHook::new_with_embed(context.config.get_webhook_url().as_str(), embed);
    //let mut webhook = DiscordWebHook::new(conf.get_webhook_url().as_str(), avatar_url.as_str());
    //webhook.add_embed(embed);
    webhook.set_avatar_url(avatar_url.as_str());
    webhook.set_username(context.heroes.get_hero_name_by_id(context.active_hero_id.clone()).as_str());
    let webhook_result = webhook.fire();
       
    if webhook_result.is_err() {
        display_error("Discord Webhock Error", &webhook_result.err().unwrap());
    }
}

fn display_error(title: &str, error: &dyn Error) {
    let dialog = Dialog::with_buttons::<gtk::Window>(
        Some(title),
        None,
        DialogFlags::MODAL,
        &[("Ok", ResponseType::Ok)]
    );
    dialog.set_modal(true);

    let dialog_label = gtk::Label::new(Some(error.to_string().as_str()));
    dialog.get_content_area().add(&dialog_label);   
    
    let webhook_url_entry = gtk::Entry::new();              
    webhook_url_entry.set_activates_default(true);
    dialog.set_default_response(ResponseType::Ok);
    dialog.get_content_area().add(&webhook_url_entry);     
    dialog.show_all();  

    let response_type = dialog.run();
    if response_type == ResponseType::Ok {
        dialog.close();
    }
      
}

fn request_webhook_url_from_user() -> String {
    let dialog = Dialog::with_buttons::<gtk::Window>(
        Some("No webhook URL was found in the config.toml."),
        None,
        DialogFlags::MODAL,
        &[("Speichern", ResponseType::Apply)]
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
    if response_type != ResponseType::Apply {
        dialog.close();
        abort_app();        
        return String::default();
    }
    let text = webhook_url_entry.get_text().to_string().trim().to_string();
    if text.is_empty() {
        abort_app();
    }
    dialog.close();
    return text;    
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

fn build_skill_name_label(skill_id: &str, skill: &JsonValue) -> gtk::Label {
    let lbl_skill_name = gtk::Label::new(skill["name"].as_str());
    lbl_skill_name.set_widget_name(format!("{}#label", skill_id).as_str());
    lbl_skill_name.set_halign(gtk::Align::Start);
    lbl_skill_name
}

fn build_test_button(context: &Context, skill_id: &str) -> gtk::Button {
    let btn_die = gtk::Button::with_label("🎲");
    btn_die.set_widget_name(format!("{}#button", skill_id).as_str());
    
    btn_die.connect_clicked(clone!(context => move |but| {
        //let hero_id = get_hero_id(&but);
        let skill_id = get_skill_id(&but);
        let difficulty = get_test_difficulty(&but);
        role_test(&context, &skill_id, difficulty);
    }));
    return btn_die;
}

fn build_checks_label(skill_id: &String, context: &Context) -> gtk::Label {
    let attribute_ids = context.skills.by_id(skill_id).get_check();
    let check_name_abbr = context.attributes.get_name_abbrs(attribute_ids);
    
    let lbl_skill_test = gtk::Label::new(Some(check_name_abbr.join(" / ").as_str()));
    lbl_skill_test.set_justify(gtk::Justification::Right);
    lbl_skill_test.set_property_width_request(100);
    lbl_skill_test
}

fn build_difficulty_entry(skill_id: &str) -> gtk::Entry {
    let en_skill_test_difculty = gtk::Entry::new();
    en_skill_test_difculty.set_widget_name(format!("{}#difficulty", skill_id).as_str());
    en_skill_test_difculty.set_alignment(0.5);
    en_skill_test_difculty.set_placeholder_text(Some("+/-"));
    en_skill_test_difculty.set_width_chars(4);
    en_skill_test_difculty.set_max_length(4);
    en_skill_test_difculty
}

fn build_hero_select(context: &Context) -> gtk::ComboBoxText {
    let hero_list = context.heroes.get_simple_hero_list();
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
    if context.active_hero_id.is_empty() {
        hero_select.set_active(Some(0));
    } else {
        hero_select.set_active_id(Some(context.active_hero_id.as_str()));
    }
    
    return hero_select;
}

fn get_skill_id(button: &gtk::Button) -> String {
    let btn_name = button.get_widget_name();
    let btn_name_split = btn_name.as_str().split('#').collect::<Vec<_>>();
    return btn_name_split[0].to_string();
}

fn get_test_difficulty(button: &gtk::Button) -> i32 {
    let skill_id = get_skill_id(button);
    let parent_widget = button
        .get_parent()
        .expect("Error: Failed to get parent widget of pressed button.");
    let widget_search_name = format!("{}#difficulty", skill_id);
    let skill_label: gtk::Entry = find_child_by_name(&parent_widget, widget_search_name.as_str())
        .expect("Error: Failed to find child");
    return skill_label
        .get_text()
        .to_string()
        .parse::<i32>()
        .or::<i32>(Ok(0))
        .unwrap();
}

fn role_test(context: &Context, skill_id: &String, difficulty: i32) {
    let mut factory = SkillCheckFactory::new(context);
    let mut ability_check = factory.get_skill_check(context.active_hero_id.clone(), skill_id.to_owned());
    let check_result = ability_check.check_ability(&difficulty);
   
    fire_webhook(&context, check_result);
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