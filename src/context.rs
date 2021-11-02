//use glib::{Cast, IsA, Object};
//use gtk::{Bin, BinExt, Container, ContainerExt, Widget, WidgetExt};

use gtk::prelude::ContainerExt;

use crate::{config::Config, difficulty::Difficulty, optolith::characters::OptolithCharacters, optolith::attributes::OptolithAttributes, optolith::skills::OptolithSkills};
#[derive(Debug, Clone)]
pub struct Context {
    pub(crate) config: Config,
    pub(crate) characters: OptolithCharacters,
    pub(crate) attributes: OptolithAttributes,
    pub(crate) skills: OptolithSkills,
    pub(crate) difficulty: Difficulty,
    pub(crate) gtk_window: Option<gtk::Window>,
    pub(crate) gtk_main_box: Option<gtk::Box>,
    pub(crate) gtk_notebook: Option<gtk::Notebook>,
    pub(crate) combat_techniques: crate::optolith::combat_techniques::OptolithCombatTechniques,
    pub(crate) gtk_avatar: Option<gtk::Image>,
    pub(crate) gtk_character_status_box: Option<gtk::Box>,
    pub(crate) gtk_window_allocation: Option<gtk::Allocation>,
    pub(crate) gtk_window_is_maximaized: bool,
    pub(crate) gtk_window_is_fullscreen: bool,
}
impl Context {
    pub fn set_notebook(&mut self, notebook: gtk::Notebook) {
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