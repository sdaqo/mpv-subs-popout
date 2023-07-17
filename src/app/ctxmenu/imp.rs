
use gtk::prelude::*;
use gtk::gdk;

pub struct ContextMenu {
    menu: gtk::Menu
}


impl ContextMenu {
    pub fn new() -> Self {
        let menu = gtk::Menu::new();
        menu.connect_popup_menu(move |menu| {
            menu.popup_easy(3, gtk::current_event_time());
            return true;
        });

        Self { menu }
    }

    pub fn add_item<W: IsA<gtk::Widget>>(&self, widget: &W, callback: Box<dyn Fn(&W, &gdk::EventButton) -> Inhibit>) {
        let item = gtk::MenuItem::new();
        item.add(widget);
        
        let widget_clone = widget.clone();
        item.connect_button_press_event(move |_wg, ev| {
            callback(&widget_clone, ev)
        });

        self.menu.append(&item);
        item.show_all();
    }

    pub fn attach_to_window<W: IsA<gtk::Widget> + IsA<gtk::Window>>(&self, window: &W) {
        let cloned_menu = self.menu.clone();
        
        window.connect_button_press_event(move |_, event| {
            if event.button() == gdk::BUTTON_SECONDARY {
                cloned_menu.popup_easy(event.button(), event.time());
                Inhibit(true)
            } else {
                Inhibit(false)
            }
        });

    }
}