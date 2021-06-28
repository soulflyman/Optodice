use gdk_pixbuf::Pixbuf;
use glib::{Cast, IsA, Object};
use gtk::{Bin, Container, Widget};
use gtk::prelude::{BinExt, ContainerExt, EntryExt, GtkWindowExt, WidgetExt};

use crate::context::Context;

pub mod settings;
pub mod dialog;
pub mod builder;
pub mod actions;

pub fn set_icon(window: &gtk::Window) {
    let window_icon = Pixbuf::from_file("./dice-shield.png");
    if window_icon.is_ok() {
        window.set_icon(Some(&window_icon.unwrap()));
    }        
}

pub fn get_check_difficulty(button: &gtk::Button, difficulty_widget_name: &String) -> i32 {
    //let skill_id = get_skill_id(&button.clone().upcast::<gtk::Widget>());
    let parent_widget = button
        .parent()
        .expect("Error: Failed to get parent widget of pressed button.");    
    let skill_label: gtk::Entry = find_child_by_name(&parent_widget, difficulty_widget_name.as_str())
        .expect("Error: Failed to find child");
    return skill_label
        .text()
        .to_string()
        .parse::<i32>()
        .or::<i32>(Ok(0))
        .unwrap();
}


pub fn clear_notebook(context: &mut Context) {
    let old_notebook = context.gtk_notebook.clone();
    if old_notebook.is_some() {
        context.gtk_main_box.as_ref().unwrap().remove(&old_notebook.unwrap())
    }
    let new_notebook = gtk::Notebook::new();
    new_notebook.set_widget_name("hero_stats");
    new_notebook.set_halign(gtk::Align::Fill);
    new_notebook.set_valign(gtk::Align::Fill);
    new_notebook.set_vexpand(true);
    context.set_notebook(new_notebook);
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
        for child in container.children() {
            if child.widget_name() == name {
                return Some(child);
            }
            if let Some(widget) = find_widget_by_name(&child, name) {
                return Some(widget);
            }
        }
    } else if let Ok(bin) = parent.clone().dynamic_cast::<Bin>() {
        if let Some(child) = bin.child() {
            if child.widget_name() == name {
                return Some(child);
            }
            if let Some(widget) = find_widget_by_name(&child, name) {
                return Some(widget);
            }
        }
    }
    None
}
