mod optolith;
mod checks;
mod config;
mod context;
mod difficulty;
mod ui;
mod webhook;
mod avatar;
mod cache;

use gio::prelude::*;
use glib::clone;
use gtk::{Application, PackType, prelude::*};
use std::{cell::RefCell, rc::Rc};

use crate::config::Config;
use crate::context::Context;
use crate::optolith::{heroes::*, attributes::*, combat_techniques::*, skills::*};
use crate::checks::{skill_check_factory::*, results::*};
use crate::difficulty::Difficulty;
use crate::ui::actions::change_hero;
use crate::ui::builder::build_hero_select;
use crate::ui::set_icon;
use crate::ui::settings::display_config;
use crate::ui::dialog::*;

#[macro_use] extern crate serde_derive;

#[derive(Debug, Clone)]
pub struct CheckFactories {
    skills: SkillCheckFactory,    
}

fn main() {

    //debug GTK ui: GTK_DEBUG=interactive cargo run
    let context: Rc<RefCell<Context>> = Rc::new(RefCell::new(Context {
        config: Config::load(),
        heroes: OptolithHeroes::new(),
        attributes: OptolithAttributes::new(),
        skills: OptolithSkills::new(),
        difficulty: Difficulty::default(),
        combat_techniques: OptolithCombatTechniques::new(),
        gtk_window: None,
        gtk_main_box: None,
        gtk_notebook: None,
        gtk_avatar: None,
        gtk_hero_status_box: None,
    }));
   
    //TODO use check_factories in button actions!
    //let check_factories = CheckFactories {
    //    skills: SkillCheckFactory::new(&context.borrow()),
    //};    

    let last_used_hero_id = context.borrow().config.last_used_hero_id().clone();
    context.borrow_mut().heroes.set_active_hero(last_used_hero_id);
    
    let app = Application::new(
        Some("net.farting-unicorn.optodice"),
        gio::ApplicationFlags::FLAGS_NONE,
    );
 
    //todo check if it makes sense to use bind_property anywhere in the project
    // https://github.com/gtk-rs/gtk-rs/blob/ebf86fe9e5e5c0bb43437a88b84928b3466cd45b/examples/src/bin/listbox_model.rs#L128
    // https://gtk-rs.org/docs/gtk/struct.ComboBoxText.html#method.bind_property
    app.connect_activate(clone!(@weak context => move |app| {
        let main_window = gtk::WindowBuilder::new().build();
        set_icon(&main_window);        
        context.borrow_mut().gtk_window = Some(main_window);
        
        check_config(&mut context.borrow_mut());
        
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        
        let cbt_hero_select = build_hero_select(&mut context.borrow_mut());        
        cbt_hero_select.connect_changed(clone!(@weak context => move |hero_select| {
            change_hero(&context, &hero_select);                   
        }));
                
        let box_hero = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        box_hero.add(&cbt_hero_select);
        box_hero.set_child_packing(&cbt_hero_select,true,true, 0, PackType::Start);

        let config_button_label = String::from("⚙️");
        let config_button = gtk::Button::with_label(&config_button_label);
        config_button.connect_clicked(clone!(@weak context => move |_| {
            display_config(&context);
        }));
        box_hero.add(&config_button);
        
        context.borrow_mut().gtk_avatar = Some(gtk::Image::new());
        context.borrow_mut().gtk_avatar.as_ref().unwrap().set_halign(gtk::Align::End);
        context.borrow_mut().gtk_avatar.as_ref().unwrap().set_widget_name("optolith_avatar");
        
       

        main_box.add(&box_hero);
               
        let hero_status_box_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        main_box.add(&hero_status_box_container);
        context.borrow_mut().gtk_hero_status_box = Some(hero_status_box_container);
        
        context.borrow_mut().gtk_window.as_ref().unwrap().set_title("Optodice");
        context.borrow_mut().gtk_window.as_ref().unwrap().add(&main_box);
        context.borrow_mut().gtk_window.as_ref().unwrap().set_application(Some(app));
        context.borrow_mut().gtk_window.as_ref().unwrap().show_all();

        context.borrow_mut().gtk_main_box = Some(main_box);
        
        change_hero(&context, &cbt_hero_select);
    }));

    app.run();
}

fn check_config(context: &mut Context) {
    
    if !context.config.is_webhook_url_set() {
        context.config.set_webhook_url(request_webhook_url());
    }

    if !context.config.is_avatar_uploader_url_set() {
        context.config.set_avatar_uploader_url(request_avatar_uploader_url());
    }
}

