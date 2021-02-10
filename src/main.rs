mod optolith;

use discord_webhook::DiscordWebHook;
use gio::prelude::*;
use glib::{Cast, IsA, Object};
use gtk::{
    prelude::*, Application, Bin, ButtonsType, Container, DialogFlags, MessageDialog, MessageType,
    Widget,
};
use json::JsonValue;
use std::{env, fs};

use crate::optolith::optolith::*;

fn main() {
    //debug GTK ui: GTK_DEBUG=interactive cargo run

    let path = "./src/talents.json";
    let json_data = fs::read_to_string(path).expect("Unable to read file");
    let talents: JsonValue = json::parse(&json_data).expect("Error: Parsing of json data failed.");

    let heroes = OptolithHeroes::new();

    let app = Application::new(
        Some("net.farting-unicorn.optodice"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Failed to initialize GTK.");
    app.connect_activate(move |app| {
        let box_main = gtk::Box::new(gtk::Orientation::Vertical, 10);

        let cbt_hero_select = build_hero_select(heroes.get_simple_hero_list());
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

                let en_talent_test_difculty = build_dificulty_entry(&talent_id);
                box_talent.add(&en_talent_test_difculty);

                let btn_die = build_test_button(&talent_id);
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
    });

    app.run(&env::args().collect::<Vec<_>>());
}

fn build_talent_name_label(talent_id: &str, talent: &JsonValue) -> gtk::Label {
    let lbl_talent_name = gtk::Label::new(talent["name"].as_str());
    lbl_talent_name.set_widget_name(format!("{}#label", talent_id).as_str());
    lbl_talent_name.set_halign(gtk::Align::Start);
    lbl_talent_name
}

fn build_test_button(talent_id: &str) -> gtk::Button {
    let btn_die = gtk::Button::with_label("🎲");
    btn_die.set_widget_name(format!("{}#button", talent_id).as_str());
    btn_die.connect_clicked(move |but| {
        let hero_id = get_hero_id(&but);
        let talent_id = get_talent_id(&but);
        let dificulty = get_test_dificulty(&but);
        role_test(hero_id, talent_id, dificulty);
    });
    btn_die
}

fn build_test_label(talent: &JsonValue) -> gtk::Label {
    let mut talent_test_label_text = talent["test"][0].to_string();
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

fn build_dificulty_entry(talent_id: &str) -> gtk::Entry {
    let en_talent_test_difculty = gtk::Entry::new();
    en_talent_test_difculty.set_widget_name(format!("{}#dificulty", talent_id).as_str());
    en_talent_test_difculty.set_alignment(0.5);
    en_talent_test_difculty.set_placeholder_text(Some("+/-"));
    en_talent_test_difculty.set_width_chars(4);
    en_talent_test_difculty.set_max_length(4);
    en_talent_test_difculty
}

fn build_hero_select(hero_list: Vec<SimpleHero>) -> gtk::ComboBoxText {
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
    hero_select.set_active(Some(0));
    hero_select.set_widget_name("hero_select");
    //hero_select.set_active_id(Some("H_1608428409833"));
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

fn get_test_dificulty(button: &gtk::Button) -> i32 {
    let talent_id = get_talent_id(button);
    let parent_widget = button
        .get_parent()
        .expect("Error: Failed to get parent widget of pressed button.");
    let widget_search_name = format!("{}#dificulty", talent_id);
    let talent_label: gtk::Entry = find_child_by_name(&parent_widget, widget_search_name.as_str())
        .expect("Error: Failed to find child");
    return talent_label
        .get_text()
        .to_string()
        .parse::<i32>()
        .or::<i32>(Ok(0))
        .unwrap();
}

fn role_test(hero_id: String, id: String, dificulty: i32) {
    println!(
        "Hero {} roles test for {} with dificulty {}",
        hero_id, id, dificulty
    );
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
