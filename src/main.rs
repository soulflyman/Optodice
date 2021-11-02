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
use crate::optolith::{characters::*, attributes::*, combat_techniques::*, skills::*};
use crate::checks::{skill_check_factory::*, results::*};
use crate::difficulty::Difficulty;
use crate::ui::actions::change_character;
use crate::ui::builder::build_character_select;
use crate::ui::set_icon;
use crate::ui::settings::display_config;
use crate::ui::dialog::*;

//#[macro_use] extern crate serde_derive;

#[derive(Debug, Clone)]
pub struct CheckFactories {
    skills: SkillCheckFactory,    
}

const APP_NAME: &str = "optodice";

fn main() {
    //debug GTK ui: GTK_DEBUG=interactive cargo run

    //TODO use winres to diplay version info and file icon in windows (https://docs.rs/winres/0.1.11/winres/)

    let context: Rc<RefCell<Context>> = Rc::new(RefCell::new(Context {
        config: Config::load(),
        characters: OptolithCharacters::new(),
        attributes: OptolithAttributes::new(),
        skills: OptolithSkills::new(),
        difficulty: Difficulty::default(),
        combat_techniques: OptolithCombatTechniques::new(),
        gtk_window: None,
        gtk_main_box: None,
        gtk_notebook: None,
        gtk_avatar: None,
        gtk_character_status_box: None,
        gtk_window_allocation: None,
        gtk_window_is_maximaized: false,
        gtk_window_is_fullscreen: false,
    }));
   
    

    //TODO use check_factories in button actions!
    //let check_factories = CheckFactories {
    //    skills: SkillCheckFactory::new(&context.borrow()),
    //};    

    let last_used_character_id = context.borrow().config.last_used_character_id().clone();
    context.borrow_mut().characters.set_active_character(last_used_character_id);
    
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
        main_window.connect_size_allocate(clone!(@weak context => move | _, alloc | {
            window_size_changed(&context, alloc.to_owned());            
        }));
        context.borrow_mut().gtk_window = Some(main_window);
                
        check_config(&mut context.borrow_mut());
        
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        
        let cbt_character_select = build_character_select(&mut context.borrow_mut());        
        cbt_character_select.connect_changed(clone!(@weak context => move |character_select| {
            change_character(&context, &character_select);                   
        }));
                
        let box_character = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        box_character.add(&cbt_character_select);
        box_character.set_child_packing(&cbt_character_select,true,true, 0, PackType::Start);

        let config_button_label = String::from("⚙️");
        let config_button = gtk::Button::with_label(&config_button_label);
        config_button.connect_clicked(clone!(@weak context => move |_| {
            display_config(&context);
        }));
        box_character.add(&config_button);
        
        context.borrow_mut().gtk_avatar = Some(gtk::Image::new());
        context.borrow_mut().gtk_avatar.as_ref().unwrap().set_halign(gtk::Align::End);
        context.borrow_mut().gtk_avatar.as_ref().unwrap().set_widget_name("optolith_avatar");
        
       

        main_box.add(&box_character);
               
        let charcter_status_box_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        main_box.add(&charcter_status_box_container);
        context.borrow_mut().gtk_character_status_box = Some(charcter_status_box_container);
        
        context.borrow_mut().gtk_window.as_ref().unwrap().set_title("Optodice");
        context.borrow_mut().gtk_window.as_ref().unwrap().add(&main_box);
        context.borrow_mut().gtk_window.as_ref().unwrap().set_application(Some(app));
        context.borrow_mut().gtk_window.as_ref().unwrap().show_all();

        context.borrow_mut().gtk_main_box = Some(main_box);
        
        change_character(&context, &cbt_character_select);
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

fn window_size_changed(context: &Rc<RefCell<Context>>, alloc: gtk::Allocation) {    
    if let Ok(mut cx) = context.try_borrow_mut() {
        cx.gtk_window_allocation = Some(alloc.to_owned());
        dbg!(&alloc);
        dbg!(&cx.gtk_window_allocation);
    }
}


