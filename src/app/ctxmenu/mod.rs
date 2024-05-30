mod imp;

use gtk::gdk;
use gtk::glib;
use gtk::glib::clone;
use gtk::{
    prelude::*, subclass::prelude::ObjectSubclassIsExt, CheckButton, ColorChooserDialog,
    FontChooserDialog, Label,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::app::utils::get_style_string;
use crate::app::channel::Message;
use crate::app::reference_dialog::ReferenceDialog;
use crate::app::MpvSubsWindow;
use crate::config::AppConfig;


use imp::ContextMenu;

pub fn build_ctxmenu(window: &MpvSubsWindow) -> ContextMenu {
    let ctxmenu = ContextMenu::new();

    let config = AppConfig::new();

    let ontop_btn = CheckButton::builder()
        .label("Always on Top")
        .active(config.ontop)
        .build();

    ctxmenu.add_item(
        &ontop_btn,
        Box::new(
            clone!(@weak window => @default-return Inhibit(true), move |wg, _ev|  {
                let state = wg.is_active();

                wg.set_active(!state);
                window.set_keep_above(!state);

                let mut config = AppConfig::new();
                config.ontop = !state;
                config.save();

                Inhibit(true)
            }),
        ),
        None,
    );

    let dock_btn = CheckButton::builder()
        .label("Docked")
        .active(config.docked)
        .build();

    ctxmenu.add_item(
        &dock_btn,
        Box::new(
            clone!(@weak window => @default-return Inhibit(true), move |wg, _ev| {
                let state = wg.is_active();
                if state {
                    wg.set_active(false);
                    window.set_type_hint(gdk::WindowTypeHint::Normal);
                } else {
                    wg.set_active(true);
                    window.set_type_hint(gdk::WindowTypeHint::Dock);
                }

                let mut config = AppConfig::new();
                config.docked = !state;
                config.save();

                Inhibit(true)
            }),
        ),
        None,
    );

    let borders_btn = CheckButton::builder()
        .label("Borders")
        .active(config.borders)
        .build();

    ctxmenu.add_item(
        &borders_btn,
        Box::new(
            clone!(@weak window => @default-return Inhibit(true), move |wg, _ev| {
                let state = wg.is_active();

                wg.set_active(!state);
                window.set_decorated(!state);
            
                let mut config = AppConfig::new();
                config.borders = !state;
                config.save();

                Inhibit(true)
            }),
        ),
        None,
    );

    let sizelock_btn = CheckButton::builder()
        .label("Lock Size")
        .active(config.size_lock.is_some())
        .build();

    ctxmenu.add_item(
        &sizelock_btn,
        Box::new(
            clone!(@weak window => @default-return Inhibit(true), move |wg, _ev| {
                let state = wg.is_active();

                wg.set_active(!state);
                
                let mut config = AppConfig::new();
                let label_box = window.imp().label_box.get().unwrap();
                if !state {
                    let width = label_box.allocated_width();
                    let height = label_box.allocated_height();

                    label_box.set_size_request(width, height);
                    window.imp().sub_label.get().unwrap().set_wrap(true);
                    window.imp().sub_label.get().unwrap().set_wrap(true);

                    config.size_lock = Some((width, height));
                } else {
                    label_box.set_size_request(0, 0);
                    window.imp().sub_label.get().unwrap().set_wrap(false);
                    window.imp().sub_label.get().unwrap().set_wrap(false);
                    config.size_lock = None;
                }

                config.save();
                Inhibit(true)
            }),
        ),
        None,
    );

    let auto_tl_btn = CheckButton::builder()
        .label("Auto Translate")
        .active(config.auto_tl)
        .build();
    
    ctxmenu.add_item(
        &auto_tl_btn, 
        Box::new(
            clone!(@weak window => @default-return Inhibit(true), move |wg, _ev| {
                let state = wg.is_active();

                wg.set_active(!state);

                let _  = window.imp().channel_sender.get().unwrap().send(Message::SetTlLabelVisibilty(!state));
                let _  = window.imp().channel_sender.get().unwrap().send(Message::UpdateTlLabel("".to_string()));

                let mut config = AppConfig::new();
                if let Some(size) = config.size_lock {
                    window.imp().label_box.get().unwrap().set_size_request(size.0, size.1);
                    window.imp().sub_label.get().unwrap().set_wrap(true);
                    window.imp().sub_label.get().unwrap().set_wrap(true);
                }

                config.auto_tl = !state;
                config.save();

                Inhibit(false)
            })
        ),
        None
    );



    let font = Label::new(Some("Change Font"));
    font.set_xalign(0.0);

    ctxmenu.add_item(
        &font,
        Box::new(
            clone!(@weak window => @default-return Inhibit(true), move |_wg, _ev| {
                let font_chooser = FontChooserDialog::new(
                    Some("Choose a font"),
                    Some(&window),
                );

                let cfg = AppConfig::new();
                font_chooser.set_font(&format!("{} {}", cfg.font_family, cfg.font_size));

                let res = font_chooser.run();
                if res != gtk::ResponseType::Ok {
                    font_chooser.close();
                    return Inhibit(true);
                }


                if let Some(font_desc) = font_chooser.font_desc() {
                    let family = font_desc.family().unwrap_or_default().to_string();
                    let size = font_desc.size() / gtk::pango::SCALE;

                    let mut cfg = AppConfig::new();
                    cfg.font_family = family;
                    cfg.font_size = size;
                    cfg.save();

                    let style_str = get_style_string(&cfg);
                    window.imp().css_provider.get().unwrap().load_from_data(&style_str).ok();
                }

                font_chooser.close();

                Inhibit(true)
            }),
        ),
        None,
    );

    let bg = Label::new(Some("Change BG Color"));
    bg.set_xalign(0.0);

    ctxmenu.add_item(
        &bg,
        Box::new(
            clone!(@weak window => @default-return Inhibit(true), move |_wg, _ev| {
                let color_chooser = ColorChooserDialog::new(
                    Some("Choose a BG Color"),
                    Some(&window)
                );

                let res = color_chooser.run();

                if res != gtk::ResponseType::Ok {
                    color_chooser.close();
                    return Inhibit(true);
                }

                let mut cfg = AppConfig::new();
                cfg.bg_col = color_chooser.rgba().to_string();
                cfg.save();

                let style_str = get_style_string(&cfg);
                window.imp().css_provider.get().unwrap().load_from_data(&style_str).ok();

                color_chooser.close();

                Inhibit(true)
            }),
        ),
        None,
    );

    let text_col = Label::new(Some("Change Text Color"));
    text_col.set_xalign(0.0);

    ctxmenu.add_item(
        &text_col,
        Box::new(
            clone!(@weak window => @default-return Inhibit(true), move |_wg, _ev| {
                let color_chooser = ColorChooserDialog::new(
                    Some("Choose a Text Color"),
                    Some(&window)
                );

                let res = color_chooser.run();

                if res != gtk::ResponseType::Ok {
                    color_chooser.close();
                    return Inhibit(true);
                }

                let mut cfg = AppConfig::new();
                cfg.text_col = color_chooser.rgba().to_string();
                cfg.save();

                let style_str = get_style_string(&cfg);
                window.imp().css_provider.get().unwrap().load_from_data(&style_str).ok();
                color_chooser.close();

                Inhibit(true)
            }),
        ),
        None,
    );

    let reset = Label::new(Some("Reset"));
    reset.set_xalign(0.0);

    ctxmenu.add_item(&reset, Box::new(clone!(@weak window, @weak ontop_btn, @weak dock_btn, @weak borders_btn, @weak auto_tl_btn => @default-return Inhibit(true), move |_wg, _ev| {
        let cfg_path = AppConfig::config_dir();
        cfg_path.delete().ok();

        let cfg = AppConfig::new();

        let style_str = get_style_string(&cfg);
        window.imp().css_provider.get().unwrap().load_from_data(&style_str).ok();

        window.set_keep_above(cfg.ontop);
        window.set_decorated(cfg.borders);
        ontop_btn.set_active(cfg.ontop);
        borders_btn.set_active(cfg.borders);
        auto_tl_btn.set_active(cfg.auto_tl);

        let _  = window.imp().channel_sender.get().unwrap().send(Message::SetTlLabelVisibilty(cfg.auto_tl));

        if cfg.docked {
            window.set_type_hint(gdk::WindowTypeHint::Dock);
            dock_btn.set_active(true);
        } else {
            window.set_type_hint(gdk::WindowTypeHint::Normal);
            dock_btn.set_active(false);
        }

        Inhibit(true)
    })), None);

    let trans_dict = Label::new(Some("Translate/Lookup"));
    trans_dict.set_xalign(0.0);

    ctxmenu.add_item(
        &trans_dict,
        Box::new(
            clone!(@weak window => @default-return Inhibit(true), move |_wg, _ev| {
                let reference_dialog = ReferenceDialog::new(&window);

                if let Some(label) = window.imp().sub_label.get() {
                    if let Some(bounds) = label.selection_bounds() {
                        let (start, end) = bounds;
                        reference_dialog.translator.set_text_and_tl(
                            &label
                                .text()
                                .as_str()
                                .graphemes(true)
                                .collect::<Vec<&str>>()[start as usize..end as usize]
                                .join("")
                        )
                    }
                };

                reference_dialog.run();
                Inhibit(true)
            }),
        ),
        None
    );

    let quit = Label::new(Some("Quit"));
    quit.set_xalign(0.0);

    ctxmenu.add_item(
        &quit,
        Box::new(
            clone!(@weak window => @default-return Inhibit(true), move |_wg, _ev| {
                window.quit();
                Inhibit(true)
            }),
        ),
        None,
    );

    ctxmenu
}
