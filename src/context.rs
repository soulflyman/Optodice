//use glib::{Cast, IsA, Object};
//use gtk::{Bin, BinExt, Container, ContainerExt, Widget, WidgetExt};

use gtk::ContainerExt;

use crate::{config::Config, optolith_attributes::OptolithAttributes, optolith_heroes::optolith::OptolithHeroes, optolith_skills::OptolithSkills, difficulty::Difficulty};
#[derive(Debug, Clone)]
pub struct Context {
    pub config: Config,
    pub heroes: OptolithHeroes,
    pub attributes: OptolithAttributes,
    pub skills: OptolithSkills,
    pub difficulty: Difficulty,
    pub gtk_window: Option<gtk::Window>,
    pub gtk_main_box: Option<gtk::Box>,
    pub gtk_notebook: Option<gtk::Notebook>,
}
impl Context {
    pub fn add_notebook(&mut self, notebook: gtk::Notebook) {
        self.gtk_main_box.as_ref().unwrap().add(&notebook);
        self.gtk_notebook = Some(notebook);
    }

    /*
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
    fn find_child_by_name<C: IsA<Widget>>(
        &self,
        name: &str,
    ) -> Option<C> {
        self.find_widget_by_name(self.gtk_window.as_ref().unwrap(), name).and_then(|widget| widget.downcast().ok())
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
    fn find_widget_by_name<W: Clone + IsA<Object> + IsA<Widget>>(&self,
        parent: &W,
        name: &str,
    ) -> Option<Widget> {
        if let Ok(container) = parent.clone().dynamic_cast::<Container>() {
            for child in container.get_children() {
                if child.get_widget_name() == name {
                    return Some(child);
                }
                if let Some(widget) = self.find_widget_by_name(&child, name) {
                    return Some(widget);
                }
            }
        } else if let Ok(bin) = parent.clone().dynamic_cast::<Bin>() {
            if let Some(child) = bin.get_child() {
                if child.get_widget_name() == name {
                    return Some(child);
                }
                if let Some(widget) = self.find_widget_by_name(&child, name) {
                    return Some(widget);
                }
            }
        }
        None
    }
    */
}