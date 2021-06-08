use std::{cell::RefCell, rc::Rc};

use glib::clone;
use gtk::{EntryExt, prelude::*};


use crate::{context::Context, ui::set_icon};

pub fn display_config(context: &Rc<RefCell<Context>>) {
    let config = &mut context.borrow_mut().config;
    let glade_src = include_str!("./../../settings_layout.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let config_window: gtk::Window = builder.get_object("config_window").unwrap();
    config_window.set_title("Optodice - Einstellungen");
    set_icon(&config_window);

    let close_button: gtk::Button = builder.get_object("config#close_button").unwrap();
    close_button.connect_clicked(clone!(@weak config_window => move |_| {        
        config_window.close();
    }));

    let webhook_url_entry: gtk::Entry = builder.get_object("config#discord#webhook_url").unwrap();
    webhook_url_entry.set_text(config.webhook_url().as_str());
    webhook_url_entry.connect_changed(clone!(@weak context => move |ui_entry| {
        let url = ui_entry.get_text().to_string();
        context.borrow_mut().config.set_webhook_url(url);
    }));

    let avatar_static_url_entry: gtk::Entry = builder.get_object( "config#avatar#static_url").unwrap();
    avatar_static_url_entry.set_text(config.avatar_static_url().as_str());
    avatar_static_url_entry.connect_changed(clone!(@weak context => move |ui_entry| {
        let url = ui_entry.get_text().to_string();
        context.borrow_mut().config.set_avatar_static_url(url);
    }));

    let avatar_uploader_url_entry: gtk::Entry = builder.get_object( "config#avatar#uploader_url").unwrap();
    avatar_uploader_url_entry.set_text(config.avatar_uploader_url().as_str());
    avatar_uploader_url_entry.connect_changed(clone!(@weak context => move |ui_entry| {
        let url = ui_entry.get_text().to_string();
        context.borrow_mut().config.set_avatar_uploader_url(url);
    }));


    config_window.show_all();    
}