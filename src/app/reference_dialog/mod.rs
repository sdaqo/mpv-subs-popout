mod translator;

use gtk::prelude::*;
use translator::TranslatorWidget;

pub struct ReferenceDialog {
    dialog: gtk::Dialog,
    stack: gtk::Stack,
    pub translator: TranslatorWidget,
    // dict: DictionaryWidget
}

impl ReferenceDialog {
    pub fn new<W: IsA<gtk::Window>>(window: &W) -> Self {
        let dialog = gtk::Dialog::new();
        let stack = gtk::Stack::new();

        let translator = TranslatorWidget::new();
        // let dict = DictionaryWidget::new();

        stack.add_named(&translator, "translator");
        // stack.add_named(&dict, "dictionary");

        dialog.set_title("Reference");
        dialog.set_default_size(500, 400);
        dialog.set_modal(true);
        dialog.set_transient_for(Some(window));
        dialog.set_icon_name(Some("gtk-find-and-replace"));

        let ca = dialog.content_area();
        ca.set_vexpand(true);
        ca.set_hexpand(true);
        ca.pack_start(&stack, true, true, 0);

        dialog.show_all();

        Self {
            dialog,
            stack,
            translator,
            // dict
        }
    }

    pub fn run(&self) {
        self.dialog.run();
    }
}
