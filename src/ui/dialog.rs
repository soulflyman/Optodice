use std::{error::Error, process};

use gtk::{ButtonsType, Dialog, DialogFlags, MessageDialog, MessageType, ResponseType};
use gtk::prelude::{ContainerExt, EntryExt, GtkWindowExt, WidgetExt, DialogExt};


pub fn abort_app_with_message(titel: &str, message: &str) {
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

pub fn display_error(title: &str, error_msg: &str) {
    let dialog = Dialog::with_buttons::<gtk::Window>(
        Some(title),
        None,
        DialogFlags::MODAL,
        &[("Ok", ResponseType::Ok)]
    );
    dialog.set_modal(true);

    let dialog_label = gtk::Label::new(Some(error_msg));
    dialog.content_area().add(&dialog_label);
    dialog.set_default_response(ResponseType::Ok);

    dialog.show_all();  

    let response_type = dialog.run();
    if response_type == ResponseType::Ok {
        dialog.hide();
    }      
}

pub fn request_webhook_url() -> String {
    let title = "Keine Webhook URL gefunden";
    let message = "Bitte gib eine gültige Discord Webhook URL ein.\nDiese kann später noch geändert werden.";
    let apply_button_text = "Speichern";
    let webhook_url = string_request_dialog(title, message, apply_button_text);
    if webhook_url.is_empty() {
        abort_app_with_message("We need more hooks!", "No Hook, no Game!");
    }

    return webhook_url;
}


pub fn string_request_dialog(title: &str, message: &str, apply_button_text: &str) -> String {
    let dialog = Dialog::with_buttons::<gtk::Window>(
        Some(title),
        None,
        DialogFlags::MODAL,
        &[(apply_button_text, ResponseType::Apply), ("Abbruch", ResponseType::Cancel)]
    );
    dialog.set_modal(true);

    let dialog_label = gtk::Label::new(Some(message));
    dialog.content_area().add(&dialog_label);   
    
    let webhook_url_entry = gtk::Entry::new();              
    webhook_url_entry.set_activates_default(true);
    dialog.set_default_response(ResponseType::Apply);
    dialog.content_area().add(&webhook_url_entry);     
    dialog.show_all();         

    let response_type = dialog.run();
    if response_type != ResponseType::Apply {
        dialog.hide();     
        return String::default();
    }
    let text = webhook_url_entry.text().to_string().trim().to_string();    
    dialog.hide();
    return text;   
}

pub fn request_avatar_uploader_url() -> String {
    let title = "Keine Avatar Uploader Script URL gefunden";
    let message = "Die Verwendung eines Avatar Uploader Scripts ist Optional und kann später noch geändert werden.\n\nBitte gib die URL zum Avatar Uploader Script ein.";
    let apply_button_text = "Speichern";
    return string_request_dialog(title, message, apply_button_text)
}

pub fn display_reqwest_error(title: &str, error: &dyn Error) {
    let dialog = Dialog::with_buttons::<gtk::Window>(
        Some(title),
        None,
        DialogFlags::MODAL,
        &[("Ok", ResponseType::Ok)]
    );
    dialog.set_modal(true);

    let dialog_label = gtk::Label::new(Some(error.to_string().as_str()));
    dialog.content_area().add(&dialog_label);
    dialog.set_default_response(ResponseType::Ok);

    dialog.show_all();  

    let response_type = dialog.run();
    if response_type == ResponseType::Ok {
        dialog.hide();
    }      
}