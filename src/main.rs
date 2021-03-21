mod optolith_weapon;
mod optolith_hero;
mod optolith_heroes;
mod check_result;
mod config;
mod skill_check_result;
mod skill_check_factory;
mod skill_check;
mod attribute_check_result;
mod attribute_check;
mod optolith_attributes;
mod optolith_skills;
mod context;
mod difficulty;

use crate::optolith_heroes::optolith::*;
use attribute_check::AttributeCheck;
use config::Config;
use check_result::CheckResult;
use skill_check_result::SkillCheckResult;
use attribute_check_result::AttributeCheckResult;
use context::Context;
use discord_webhook::{DiscordWebHook, Embed};
use gio::prelude::*;
use glib::{Cast, IsA, Object};
use gtk::{Application, Bin, ButtonsType, Container, Dialog, DialogFlags, EntryExt, MessageDialog, MessageType, PackType, ResponseType, Widget, prelude::*};
use gdk_pixbuf::Colorspace;
use image::GenericImageView;
use std::{cell::RefCell, env, error::Error, process, rc::Rc};
use crate::skill_check_factory::SkillCheckFactory;
use crate::optolith_skills::OptolithSkills;
use crate::optolith_attributes::OptolithAttributes;
use crate::difficulty::Difficulty;

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
    let context: Rc<RefCell<Context>> = Rc::new(RefCell::new(Context {
        config: Config::load(),
        heroes: OptolithHeroes::new(),
        attributes: OptolithAttributes::new(),
        skills: OptolithSkills::new(),
        difficulty: Difficulty::default(),
        gtk_window: None,
        gtk_main_box: None,
        gtk_notebook: None,
    }));
   
    //TODO use check_factories in button actions!
    //let check_factories = CheckFactories {
    //    skills: SkillCheckFactory::new(&context.borrow()),
    //};    

    let last_used_hero_id = context.borrow().config.get_last_used_hero_id().clone();
    context.borrow_mut().heroes.set_active_hero(last_used_hero_id);
    
    let app = Application::new(
        Some("net.farting-unicorn.optodice"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Failed to initialize GTK.");
    
    //todo check if it makes sense to use bind_property anywhere in the project
    // https://github.com/gtk-rs/gtk-rs/blob/ebf86fe9e5e5c0bb43437a88b84928b3466cd45b/examples/src/bin/listbox_model.rs#L128
    // https://gtk-rs.org/docs/gtk/struct.ComboBoxText.html#method.bind_property
    app.connect_activate(clone!(context => move |app| {
        
        context.borrow_mut().gtk_window = Some(gtk::WindowBuilder::new().build());
        
        check_config(&mut context.borrow_mut());
        
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        
        let cbt_hero_select = build_hero_select(&context.borrow_mut());        
        cbt_hero_select.connect_changed(clone!(context => move |hero_select| {
            change_hero(&context, &hero_select);                   
        }));
                
        let box_hero = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        box_hero.add(&cbt_hero_select);
        box_hero.set_child_packing(&cbt_hero_select,true,true, 0, PackType::Start);
        
        let hero_image = gtk::Image::new();
        hero_image.set_halign(gtk::Align::End);
        hero_image.set_widget_name("optolith_avatar");
        box_hero.add(&hero_image);

        main_box.add(&box_hero);
        
       
        let hero_status_box = build_hero_status_box(&context);
        main_box.add(&hero_status_box);
        
        context.borrow_mut().gtk_window.as_ref().unwrap().set_title("Optodice");
        context.borrow_mut().gtk_window.as_ref().unwrap().add(&main_box);
        context.borrow_mut().gtk_window.as_ref().unwrap().set_application(Some(app));
        context.borrow_mut().gtk_window.as_ref().unwrap().show_all();

        context.borrow_mut().gtk_main_box = Some(main_box);
        
        change_hero(&context, &cbt_hero_select);
    }));

    app.run(&env::args().collect::<Vec<_>>());
}

fn check_config(context: &mut Context) {
    
    if !context.config.is_webhook_url_set() {
        context.config.set_webhook_url(request_webhook_url());
    }

    if !context.config.is_avatar_uploader_url_set() {
        context.config.set_avatar_uploader_url(request_avatar_uploader_url());
    }
}

fn ui_add_tab_battle(context: &Rc<RefCell<Context>>) {
    let lbo_weapons = gtk::ListBox::new();
    
    lbo_weapons.set_selection_mode(gtk::SelectionMode::None);

    let nb_tab_name = gtk::Label::new(Some("Kampf"));
    context.borrow_mut().gtk_notebook.as_ref().unwrap().append_page(&lbo_weapons, Some(&nb_tab_name));

    for weapon in context.borrow().heroes.active_hero().weapons() {
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        let weapon_name = gtk::Label::new(Some(weapon.name()));
        row.add(&weapon_name);
        row.set_child_packing(&weapon_name, true, true, 0, gtk::PackType::Start);

        if weapon.can_parry() {
            let parry_value = 8;
            let pa_label =  gtk::Label::new(Some("PA")); 
            row.add(&pa_label);
            let pa_value =  gtk::Label::new(Some(parry_value.to_string().as_str())); 
            row.add(&pa_value);
            let en_parry_test_difculty = build_parry_difficulty_entry(&context, &weapon.id());
            row.add(&en_parry_test_difculty);
            let btn_die = build_parry_check_button(&context, &weapon.id());
            row.add(&btn_die);

            let slash =  gtk::Label::new(Some(" / ")); 
            row.add(&slash);
        }

        let attack_value = 12;

        let at_label =  gtk::Label::new(Some("AT"));  
        row.add(&at_label);
        let at_value =  gtk::Label::new(Some(attack_value.to_string().as_str()));  
        row.add(&at_value);

        let en_attack_test_difculty = build_attack_difficulty_entry(&context, &weapon.id());
        row.add(&en_attack_test_difculty);
        let btn_die = build_attack_check_button(&context, &weapon.id());
        row.add(&btn_die);
        
        lbo_weapons.add(&row);
    }
}

fn ui_add_tab_custom(context: &Rc<RefCell<Context>>) {
    let lbo_dice = gtk::ListBox::new();
    lbo_dice.set_selection_mode(gtk::SelectionMode::None);
    let nb_tab_name = gtk::Label::new(Some("Würfel"));
    context.borrow_mut().gtk_notebook.as_ref().unwrap().append_page(&lbo_dice, Some(&nb_tab_name));
}

fn ui_add_tab_magic(context: &Rc<RefCell<Context>>) {
    let lbo_dice = gtk::ListBox::new();
    lbo_dice.set_selection_mode(gtk::SelectionMode::None);
    let nb_tab_name = gtk::Label::new(Some("Magie"));
    context.borrow_mut().gtk_notebook.as_ref().unwrap().append_page(&lbo_dice, Some(&nb_tab_name));
}

fn build_hero_status_box(context: &Rc<RefCell<Context>>) -> gtk::Box{
    let hero_status_box = gtk::Box::new(gtk::Orientation::Horizontal, 15);
    hero_status_box.set_margin_start(15);
    hero_status_box.set_margin_end(15);
    let pain = gtk::SpinButton::with_range(0.0, 4.0, 1.0);
    pain.set_alignment(0.5);
    pain.connect_changed(clone!(context => move |pain| {
        context.borrow_mut().difficulty.pain_level = pain.get_value_as_int();
    }));
    let pain_label = gtk::Label::new(Some("Schmerz"));

    hero_status_box.add(&pain_label);
    hero_status_box.add(&pain);
    return hero_status_box;
}

fn change_hero(context: &Rc<RefCell<Context>>, hero_select: &gtk::ComboBoxText) {
    let hero_id = hero_select.get_active_id().expect("Unknown hero selected, this should not happen.");            
    context.borrow_mut().config.set_last_used_hero_id(hero_id.to_string());
    context.borrow_mut().heroes.set_active_hero(hero_id.to_string());
    
    upload_avatar(&context.borrow_mut());
    change_avatar(&context.borrow_mut(), &hero_select);
    reload_hero_stats(context);
}

fn reload_hero_stats(context: &Rc<RefCell<Context>>) {
    clear_notebook(&mut context.borrow_mut());
    
    ui_add_tab_attributes(&context);
    ui_add_tabs_skills(&context);
    ui_add_tab_battle(&context);
    ui_add_tab_magic(&context);
    ui_add_tab_custom(&context);

    context.borrow_mut().gtk_main_box.as_ref().unwrap().show_all();
}

fn clear_notebook(context: &mut Context) {
    let old_notebook = context.gtk_notebook.clone();
    if old_notebook.is_some() {
        context.gtk_main_box.as_ref().unwrap().remove(&old_notebook.unwrap())
    }
    let new_notebook = gtk::Notebook::new();
    new_notebook.set_widget_name("hero_stats");
    context.add_notebook(new_notebook);
}

fn change_avatar(context: &Context, hero_select: &gtk::ComboBoxText) {
    let avatar_raw = base64::decode(&context.heroes.active_hero().avatar().split(',').collect::<Vec<&str>>()[1]);
    let mut avatar_buffer = image::load_from_memory(&avatar_raw.unwrap()).unwrap();
    avatar_buffer = avatar_buffer.resize(100, 100, image::imageops::FilterType::Lanczos3);
    let avatar_color_channels = 4; //(RGBA)
    let pixels_row_stride = (avatar_buffer.width() * avatar_color_channels + 3) & !3;
    let pixels = avatar_buffer.clone().into_rgba8().as_raw().to_owned();
    let avatar_pixbuf: gdk_pixbuf::Pixbuf = gdk_pixbuf::Pixbuf::from_mut_slice(pixels, Colorspace::Rgb, true, 8, avatar_buffer.width() as i32, avatar_buffer.height() as i32, pixels_row_stride as i32);
    let avatar: gtk::Image = find_child_by_name(&hero_select.get_parent().unwrap(), "optolith_avatar").expect("Error: Failed to find gtk::Image Widget.");
    avatar.set_from_pixbuf(Some(&avatar_pixbuf));
}

fn upload_avatar(context: &Context) {
    if context.config.is_avatar_uploader_url_set() {
        context.heroes.active_hero().upload_avatar(context.config.get_avatar_uploader_url());            
    }
}

fn ui_add_tabs_skills(context: &Rc<RefCell<Context>>) {
    let skills_by_group = &context.borrow().skills.by_group.clone(); //todo get rid of hashmap, because it does not keep the order!
    for (skill_category, skills) in skills_by_group {        
        let lbo_skills = gtk::ListBox::new();        
        lbo_skills.set_selection_mode(gtk::SelectionMode::None);

        let nb_tab_name = gtk::Label::new(Some(skill_category));
        context.borrow_mut().gtk_notebook.as_ref().unwrap().append_page(&lbo_skills, Some(&nb_tab_name));
        dbg!(&skill_category); //todo remove

        for skill in skills {
            let box_skill = gtk::Box::new(gtk::Orientation::Horizontal, 0);

            let lbl_skill_name = build_skill_name_label(&skill.name);
            box_skill.add(&lbl_skill_name);
            box_skill.set_child_packing(&lbl_skill_name, true, true, 0, gtk::PackType::Start);

            let lbl_checks = build_checks_label(&skill.id, &context.borrow());
            box_skill.add(&lbl_checks);
            
            let lbl_skill_points = gtk::Label::new(Some(context.borrow().heroes.active_hero().skill_points(&skill.id).to_string().as_str()));
            lbl_skill_points.set_halign(gtk::Align::End);
            lbl_skill_points.set_justify(gtk::Justification::Right);
            lbl_skill_points.set_property_width_request(30);
            lbl_skill_points.set_widget_name(&format!("skill_id#{}",&skill.id));
            box_skill.add(&lbl_skill_points);            

            let en_skill_test_difculty = build_skill_difficulty_entry(&context, &skill.id);
            box_skill.add(&en_skill_test_difculty);

            let btn_die = build_skill_check_button(&context, &skill.id);
            box_skill.add(&btn_die);

            lbo_skills.add(&box_skill);
        }
    }
}

fn ui_add_tab_attributes(context: &Rc<RefCell<Context>>) {
    let lbo_attributes = gtk::ListBox::new();
    lbo_attributes.set_selection_mode(gtk::SelectionMode::None);

    let nb_tab_name = gtk::Label::new(Some("Attribute"));
    context.borrow_mut().gtk_notebook.as_ref().unwrap().append_page(&lbo_attributes, Some(&nb_tab_name));

    for (attribute_id, attribute) in context.borrow().attributes.clone().all() {
        let box_attribute = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let lbl_attribute_name = build_skill_name_label(&attribute.name);
        box_attribute.add(&lbl_attribute_name);
        box_attribute.set_child_packing(&lbl_attribute_name, true, true, 0, gtk::PackType::Start);
        
        let lbl_attribute_value = gtk::Label::new(Some(context.borrow().heroes.active_hero().attribute_value(&attribute_id.to_string()).to_string().as_str()));
        lbl_attribute_value.set_halign(gtk::Align::End);
        lbl_attribute_value.set_justify(gtk::Justification::Right);
        lbl_attribute_value.set_property_width_request(30);
        lbl_attribute_value.set_widget_name(&format!("skill_id#{}",&attribute_id));
        box_attribute.add(&lbl_attribute_value);
        
        let en_atribute_test_difculty = build_attribute_difficulty_entry(&context, &attribute_id);
        box_attribute.add(&en_atribute_test_difculty);

        let btn_die = build_attribute_check_button(&context, &attribute_id);
        box_attribute.add(&btn_die);

        lbo_attributes.add(&box_attribute);
    }
}

fn fire_webhook(context: &Context, die_result: CheckResult) {
    let mut embed = Embed::default();
    embed.description = Some(die_result.message);
    if die_result.success {
        embed.color = Some(COLOR_SUCCESS);
    } else {
        embed.color = Some(COLOR_FAILURE);
    }

    let mut webhook = DiscordWebHook::new_with_embed(context.config.get_webhook_url().as_str(), embed);

    if context.config.use_avatars() {
        let mut avatar_url = context.config.get_avatar_base_url();
        if !avatar_url.ends_with("/") {
            avatar_url.push_str("/");
        }
        avatar_url.push_str(context.heroes.active_hero().get_avatar_file_name().as_str());       
        webhook.set_avatar_url(avatar_url.as_str());
    }    
    webhook.set_username(context.heroes.active_hero().name().as_str());
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
    dialog.set_default_response(ResponseType::Ok);

    dialog.show_all();  

    let response_type = dialog.run();
    if response_type == ResponseType::Ok {
        dialog.close();
    }      
}

fn request_webhook_url() -> String {
    let title = "No webhook URL was found in the config.toml.";
    let message = "Please enter the URL of the Discord Webhook.";
    let apply_button_text = "Save";
    let webhook_url = request_string_dialog(title, message, apply_button_text);
    if webhook_url.is_empty() {
        abort_app_with_message("We need more hooks!", "No Hook, no Game!");
    }

    return webhook_url;
}

fn request_avatar_uploader_url() -> String {
    let title = "No Avatar Uploader URL was found in the config.toml.";
    let message = "Please enter the URL of the Avatar Uploader PHP script.";
    let apply_button_text = "Save";
    request_string_dialog(title, message, apply_button_text)
}

fn request_string_dialog(title: &str, message: &str, apply_button_text: &str) -> String {
    let dialog = Dialog::with_buttons::<gtk::Window>(
        Some(title),
        None,
        DialogFlags::MODAL,
        &[(apply_button_text, ResponseType::Apply)]
    );
    dialog.set_modal(true);

    let dialog_label = gtk::Label::new(Some(message));
    dialog.get_content_area().add(&dialog_label);   
    
    let webhook_url_entry = gtk::Entry::new();              
    webhook_url_entry.set_activates_default(true);
    dialog.set_default_response(ResponseType::Apply);
    dialog.get_content_area().add(&webhook_url_entry);     
    dialog.show_all();         

    let response_type = dialog.run();
    if response_type != ResponseType::Apply {
        dialog.close();     
        return String::default();
    }
    let text = webhook_url_entry.get_text().to_string().trim().to_string();    
    dialog.close();
    return text;   
}

//todo add variable message
fn abort_app_with_message(titel: &str, message: &str) {
    let msg_dialog = MessageDialog::new::<gtk::Window>(
        None,
        DialogFlags::MODAL,
        MessageType::Error,
        ButtonsType::Ok,
        message,
    );
    msg_dialog.set_title(titel);
    msg_dialog.connect_response(|_, _| std::process::exit(1));
    msg_dialog.run();

    gtk::main_quit();
    process::exit(1);
}

fn build_skill_name_label(skill_name: &String) -> gtk::Label {
    let lbl_skill_name = gtk::Label::new(Some(skill_name.as_str()));    
    lbl_skill_name.set_halign(gtk::Align::Start);
    lbl_skill_name
}

fn build_parry_check_button(context: &Rc<RefCell<Context>>, weapon_id: &str) -> gtk::Button {
    let btn_die = gtk::Button::with_label("🎲");
    let widget_name = format!("parry_check_button#{}", weapon_id);
    let difficulty_widget_name = format!("parry_difficulty#{}", weapon_id);
    btn_die.set_widget_name(widget_name.as_str());
    let attribute_id_tmp = weapon_id.to_string();
    btn_die.connect_clicked(clone!(context => move |but| {
        let difficulty = get_check_difficulty(&but, &difficulty_widget_name);
        role_parry_check(&context.borrow(), &attribute_id_tmp, difficulty);
    }));
    return btn_die;
}

fn build_attack_check_button(context: &Rc<RefCell<Context>>, weapon_id: &str) -> gtk::Button {
    let btn_die = gtk::Button::with_label("🎲");
    let widget_name = format!("attack_check_button#{}", weapon_id);
    let difficulty_widget_name = format!("attack_difficulty#{}", weapon_id);
    btn_die.set_widget_name(widget_name.as_str());
    let attribute_id_tmp = weapon_id.to_string();
    btn_die.connect_clicked(clone!(context => move |but| {
        let difficulty = get_check_difficulty(&but, &difficulty_widget_name);
        role_attack_check(&context.borrow(), &attribute_id_tmp, difficulty);
    }));
    return btn_die;
}

fn build_skill_check_button(context: &Rc<RefCell<Context>>, skill_id: &str) -> gtk::Button {
    let btn_die = gtk::Button::with_label("🎲");
    let widget_name = format!("skill_check_button#{}", skill_id);
    let difficulty_widget_name = format!("skill_difficulty#{}", skill_id);
    btn_die.set_widget_name(widget_name.as_str());
    let skill_id_tmp = skill_id.to_string();    
    btn_die.connect_clicked(clone!(context => move |but| {
        let difficulty = get_check_difficulty(&but, &difficulty_widget_name);
        role_skill_check(&context.borrow(), &skill_id_tmp, difficulty);
    }));
    return btn_die;
}

fn build_attribute_check_button(context: &Rc<RefCell<Context>>, attribute_id: &str) -> gtk::Button {
    let btn_die = gtk::Button::with_label("🎲");
    let widget_name = format!("attribute_check_button#{}", attribute_id);
    let difficulty_widget_name = format!("attribute_difficulty#{}", attribute_id);
    btn_die.set_widget_name(widget_name.as_str());
    let attribute_id_tmp = attribute_id.to_string();
    btn_die.connect_clicked(clone!(context => move |but| {
        //let hero_id = get_hero_id(&but);
        //let attribute_id = get_skill_id(&but.clone().upcast::<gtk::Widget>());
        let difficulty = get_check_difficulty(&but, &difficulty_widget_name);
        role_attribute_check(&context.borrow(), &attribute_id_tmp, difficulty);
    }));
    return btn_die;
}

fn build_checks_label(skill_id: &String, context: &Context) -> gtk::Label {
    let attribute_ids = context.skills.by_id(skill_id).get_check();
    let check_name_abbr = context.attributes.name_abbrs(attribute_ids);
    
    let lbl_skill_test = gtk::Label::new(Some(check_name_abbr.join(" / ").as_str()));
    lbl_skill_test.set_justify(gtk::Justification::Right);
    lbl_skill_test.set_property_width_request(100);
    lbl_skill_test
}

fn build_skill_difficulty_entry(context: &Rc<RefCell<Context>>, skill_id: &str) -> gtk::Entry {
    let en_skill_check_difculty = gtk::Entry::new();
    en_skill_check_difculty.set_widget_name(format!("skill_difficulty#{}", skill_id).as_str());
    en_skill_check_difculty.set_alignment(0.5);
    en_skill_check_difculty.set_placeholder_text(Some("+/-"));
    en_skill_check_difculty.set_width_chars(4);
    en_skill_check_difculty.set_max_length(4);
    en_skill_check_difculty.connect_activate(clone!(context => move |entry| {
        let skill_id = get_skill_id(&entry.clone().upcast::<gtk::Widget>()); //todo remove like in build_attribute_difficulty_entry
        let difficulty = entry.get_text().to_string().parse::<i32>().or::<i32>(Ok(0)).unwrap();
        role_skill_check(&context.borrow(), &skill_id, difficulty);
    }));
    en_skill_check_difculty
}

fn build_attribute_difficulty_entry(context: &Rc<RefCell<Context>>, attribute_id: &str) -> gtk::Entry {
    let en_attribute_check_difculty = gtk::Entry::new();
    en_attribute_check_difculty.set_widget_name(format!("attribute_difficulty#{}", attribute_id).as_str());
    en_attribute_check_difculty.set_alignment(0.5);
    en_attribute_check_difculty.set_placeholder_text(Some("+/-"));
    en_attribute_check_difculty.set_width_chars(4);
    en_attribute_check_difculty.set_max_length(4);
    let attribute_id_tmp = attribute_id.to_string();
    en_attribute_check_difculty.connect_activate(clone!(context => move |entry| {        
        let difficulty = entry.get_text().to_string().parse::<i32>().or::<i32>(Ok(0)).unwrap();
        role_attribute_check(&context.borrow(), &attribute_id_tmp, difficulty);
    }));
    en_attribute_check_difculty
}

fn build_attack_difficulty_entry(context: &Rc<RefCell<Context>>, weapon_id: &str) -> gtk::Entry {
    let en_attack_difculty = gtk::Entry::new();
    en_attack_difculty.set_widget_name(format!("attack_difficulty#{}", weapon_id).as_str());
    en_attack_difculty.set_alignment(0.5);
    en_attack_difculty.set_placeholder_text(Some("+/-"));
    en_attack_difculty.set_width_chars(4);
    en_attack_difculty.set_max_length(4);
    let attribute_id_tmp = weapon_id.to_string();
    en_attack_difculty.connect_activate(clone!(context => move |entry| {        
        let difficulty = entry.get_text().to_string().parse::<i32>().or::<i32>(Ok(0)).unwrap();
        role_attribute_check(&context.borrow(), &attribute_id_tmp, difficulty);
    }));
    en_attack_difculty
}

fn build_parry_difficulty_entry(context: &Rc<RefCell<Context>>, weapon_id: &str) -> gtk::Entry {
    let en_parry_difculty = gtk::Entry::new();
    en_parry_difculty.set_widget_name(format!("parry_difficulty#{}", weapon_id).as_str());
    en_parry_difculty.set_alignment(0.5);
    en_parry_difculty.set_placeholder_text(Some("+/-"));
    en_parry_difculty.set_width_chars(4);
    en_parry_difculty.set_max_length(4);
    let attribute_id_tmp = weapon_id.to_string();
    en_parry_difculty.connect_activate(clone!(context => move |entry| {        
        let difficulty = entry.get_text().to_string().parse::<i32>().or::<i32>(Ok(0)).unwrap();
        role_attribute_check(&context.borrow(), &attribute_id_tmp, difficulty);
    }));
    en_parry_difculty
}

fn build_hero_select(context: &Context) -> gtk::ComboBoxText {
    let hero_list = context.heroes.simple_hero_list();
    if hero_list.len() == 0 {
        //todo use abort_app()
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
    for hero in hero_list {
        hero_select.append(Some(hero.id.as_str()), hero.name.as_str());
    }
    
    hero_select.set_widget_name("hero_select");
    if context.heroes.active_hero_id().is_empty() {
        hero_select.set_active(Some(0));
    } else {
        hero_select.set_active_id(Some(context.heroes.active_hero_id().as_str()));
    }
    
    return hero_select;
}

fn get_skill_id(widget: &gtk::Widget) -> String {
    let btn_name = widget.get_widget_name();
    let btn_name_split = btn_name.as_str().split('#').collect::<Vec<_>>();
    return btn_name_split[0].to_string();
}

fn get_check_difficulty(button: &gtk::Button, difficulty_widget_name: &String) -> i32 {
    //let skill_id = get_skill_id(&button.clone().upcast::<gtk::Widget>());
    let parent_widget = button
        .get_parent()
        .expect("Error: Failed to get parent widget of pressed button.");    
    let skill_label: gtk::Entry = find_child_by_name(&parent_widget, difficulty_widget_name.as_str())
        .expect("Error: Failed to find child");
    return skill_label
        .get_text()
        .to_string()
        .parse::<i32>()
        .or::<i32>(Ok(0))
        .unwrap();
}


fn role_parry_check(context: &Context, skill_id: &String, difficulty: i32) {

}

fn role_attack_check(context: &Context, skill_id: &String, difficulty: i32) {
    
}

fn role_skill_check(context: &Context, skill_id: &String, difficulty: i32) {
    let mut factory = SkillCheckFactory::new(context);
    let mut skill_check = factory.get_skill_check(skill_id.to_owned());
    let check_result = skill_check.check_skill(&difficulty);
   
    fire_webhook(&context, check_result.to_check_result());
}

fn role_attribute_check(context: &Context, attribute_id: &String, difficulty: i32) {
    println!("Role {} {}", context.attributes.by_id(&attribute_id).name, difficulty);
    let mut skill_check = AttributeCheck::new(context, attribute_id.to_owned());
    let check_result = skill_check.check(&difficulty);

    fire_webhook(&context, check_result.to_check_result());
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