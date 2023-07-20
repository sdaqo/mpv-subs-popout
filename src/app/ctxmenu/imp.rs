
use gtk::prelude::*;
use gtk::gdk;
use gtk::glib::clone;

pub struct ContextMenu {
    menu: gtk::Menu
}


impl ContextMenu {
    pub fn new() -> Self {
        let menu = gtk::Menu::new();

        Self { menu }
    }

    pub fn add_item<W: IsA<gtk::Widget>>(
        &self, widget: &W,
        callback: Box<dyn Fn(&W, &gdk::EventButton) -> Inhibit>,
        callback_should_show: Option<Box<dyn Fn(&W) -> bool>>
    ) {
        let item = gtk::MenuItem::new();
        item.add(widget);

        if let Some(cb) = callback_should_show {
            let wg_clone = widget.clone();
            item.connect_show(move |item| {
                let should_show = cb(&wg_clone);

                if should_show {
                    item.set_sensitive(true);
                } else {
                    item.set_sensitive(false);
                }
            });
        }
        
        let widget_clone = widget.clone();
        item.connect_button_press_event(move |_wg, ev| {
            callback(&widget_clone, ev)
        });

        self.menu.append(&item);
        item.show_all();
    }

    pub fn attach_to_widget<W: IsA<gtk::Widget>>(&self, widget: &W) {
        let cloned_menu = self.menu.clone();
        
        widget.connect_button_press_event(move |_, event| {
            if event.button() == gdk::BUTTON_SECONDARY {
                cloned_menu.popup_easy(event.button(), event.time());
                cloned_menu.foreach(|wg| {
                    wg.emit_by_name::<()>("show", &[]);
                });

                Inhibit(true)
            } else {
                Inhibit(false)
            }
        });

    }
}
