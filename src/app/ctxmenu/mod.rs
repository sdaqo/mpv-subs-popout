mod imp;

use gtk::{prelude::*, subclass::prelude::ObjectSubclassIsExt, Label, CheckButton, FontChooserDialog, ColorChooserDialog};
use gtk::glib::clone;
use gtk::glib;
use gtk::gdk;

use crate::app::MpvSubsWindow;
use crate::config::AppConfig;
use crate::app::utils::get_style_string;

use imp::ContextMenu;

pub fn build_ctxmenu(window: &MpvSubsWindow) -> ContextMenu {
    let ctxmenu = ContextMenu::new();

    let config = AppConfig::new();

    let ontop_btn = CheckButton::builder()
        .label("Always on Top")
        .active(config.ontop)
        .build();

    ctxmenu.add_item(&ontop_btn, Box::new(clone!(@weak window => @default-return Inhibit(true), move |wg, _ev|  {
        let state = wg.is_active();
        
        wg.set_active(!state);
        window.set_keep_above(!state);
        
        let mut config = AppConfig::new();
        config.ontop = !state;
        config.save();

        Inhibit(true)
    })));

    let dock_btn = CheckButton::builder()
        .label("Docked")
        .active(config.docked)
        .build();

    ctxmenu.add_item(&dock_btn, Box::new(clone!(@weak window => @default-return Inhibit(true), move |wg, _ev| {
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
    })));

    let borders_btn = CheckButton::builder()
        .label("Borders")
        .active(config.borders)
        .build();

    ctxmenu.add_item(&borders_btn, Box::new(clone!(@weak window => @default-return Inhibit(true), move |wg, _ev| {
        let state = wg.is_active();

        wg.set_active(!state);
        window.set_decorated(!state);

        let mut config = AppConfig::new();
        config.borders = !state;
        config.save();

        Inhibit(true)
    })));



    let font = Label::new(Some("Change Font"));
    font.set_xalign(0.0);

    ctxmenu.add_item(&font, Box::new(clone!(@weak window => @default-return Inhibit(true), move |_wg, _ev| {
        let font_chooser = FontChooserDialog::new(
            Some("Choose a font"),
            Some(&window),
        );

        let cfg = AppConfig::new();
        font_chooser.set_font(&format!("{} {}", cfg.font_family, cfg.font_size));

        font_chooser.connect_response(move |dialog, res| {
            if res != gtk::ResponseType::Ok {
                dialog.close();
                return;
            }
        
            if let Some(font_desc) = dialog.font_desc() {
                let family = font_desc.family().unwrap_or_default().to_string();
                let size = font_desc.size() / gtk::pango::SCALE;

                let mut cfg = AppConfig::new();
                cfg.font_family = family;
                cfg.font_size = size;
                cfg.save();

                let style_str = get_style_string(&cfg);
                let _ = window.imp().css_provider.get().unwrap().load_from_data(&style_str);
            }

            dialog.close();
        });
        font_chooser.run();

        Inhibit(true)
    })));

    let bg = Label::new(Some("Change BG Color"));
    bg.set_xalign(0.0);

    ctxmenu.add_item(&bg, Box::new(clone!(@weak window => @default-return Inhibit(true), move |_wg, _ev| {
        let color_chooser = ColorChooserDialog::new(
            Some("Choose a BG Color"),
            Some(&window)
        );
        
        color_chooser.connect_response(move |dialog, res| {
            if res != gtk::ResponseType::Ok {
                dialog.close();
                return;
            }

            let mut cfg = AppConfig::new();
            cfg.bg_col = dialog.rgba().to_string(); 
            cfg.save();

            let style_str = get_style_string(&cfg);
            let _ = window.imp().css_provider.get().unwrap().load_from_data(&style_str);
            dialog.close();

        });

        color_chooser.run();


        Inhibit(true)
    })));

    let text_col = Label::new(Some("Change Text Color"));
    text_col.set_xalign(0.0);

    ctxmenu.add_item(&text_col, Box::new(clone!(@weak window => @default-return Inhibit(true), move |_wg, _ev| {
        let color_chooser = ColorChooserDialog::new(
            Some("Choose a Text Color"),
            Some(&window)
        );
        
        color_chooser.connect_response(move |dialog, res| {
            if res != gtk::ResponseType::Ok {
                dialog.close();
                return;
            }

            let mut cfg = AppConfig::new();
            cfg.text_col = dialog.rgba().to_string(); 
            cfg.save();

            let style_str = get_style_string(&cfg);
            let _ = window.imp().css_provider.get().unwrap().load_from_data(&style_str);
            dialog.close();

        });

        color_chooser.run();


        Inhibit(true)
    })));

    let reset = Label::new(Some("Reset"));
    reset.set_xalign(0.0);

    ctxmenu.add_item(&reset, Box::new(clone!(@weak window, @weak ontop_btn, @weak dock_btn, @weak borders_btn => @default-return Inhibit(true), move |_wg, _ev| {
        let cfg_path = AppConfig::config_dir();
        let _ = cfg_path.delete();

        let cfg = AppConfig::new();

        let style_str = get_style_string(&cfg);
        let _ = window.imp().css_provider.get().unwrap().load_from_data(&style_str);

        window.set_keep_above(cfg.ontop);
        window.set_decorated(cfg.borders);
        ontop_btn.set_active(cfg.ontop);
        borders_btn.set_active(cfg.borders);


        if cfg.docked {
            window.set_type_hint(gdk::WindowTypeHint::Dock);
            dock_btn.set_active(true);
        } else {
            window.set_type_hint(gdk::WindowTypeHint::Normal);
            dock_btn.set_active(false);
        }

        Inhibit(true)
    })));


    let quit = Label::new(Some("Quit"));
    quit.set_xalign(0.0);
    ctxmenu.add_item(&quit, Box::new(clone!(@strong window => @default-return Inhibit(true), move |_wg, _ev| {
        window.quit();
        Inhibit(true)
    })));

    return ctxmenu;
}