use crate::cpu_ctrl;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CheckButton, ListBox, ListBoxRow, Orientation, Button, Settings, Label};

pub fn run_app() {
    let app = Application::builder()
        .application_id("com.brzkd.smartcpu")
        .build();

    app.connect_activate(|app| {

        if let Some(settings) = Settings::default() {
            settings.set_gtk_application_prefer_dark_theme(true);
        }

        build_ui(app)
    });
    app.run();
}

fn build_ui(app: &Application) {
    let enable_button = Button::with_label("Enable all");
    enable_button.connect_clicked(|_| cpu_ctrl::enable_all());

    let disable_button = Button::with_label("Disable all");
    disable_button.connect_clicked(|_| cpu_ctrl::disable_all());

    let buttons_box = gtk4::Box::new(Orientation::Horizontal, 5);
    buttons_box.append(&enable_button);
    buttons_box.append(&disable_button);

    let threads = cpu_ctrl::get_cpu_threads_count();

    let items: Vec<String> = (1..threads as usize)
    .map(|i| format!("{}", i))
    .collect();

    let list_box = ListBox::new();

    for item in items {
        let checked = cpu_ctrl::is_thread_active(&item);
        
        let thread_num: i32 = item
        .parse()
        .expect("Incorrect thread number");
        
        // Get CPU info
        let freq = cpu_ctrl::get_cpu_frequency(thread_num);
        let governor = cpu_ctrl::get_cpu_governor(thread_num);
        
        // Create label with thread info
        let label_text = if governor != "N/A" {
            format!("Thread {} - {} - {}", item, freq, governor)
        } else {
            format!("Thread {} - {}", item, freq)
        };
        
        let checkbox = CheckButton::with_label(&label_text);
        checkbox.set_active(checked);

        let checkbox_clone = checkbox.clone();

        checkbox.connect_toggled(move |_| {
            let active = checkbox_clone.is_active();
                if active {
                    cpu_ctrl::enable_thread(thread_num);
                } else {
                    cpu_ctrl::disable_thread(thread_num);
                }
        });

        let row = ListBoxRow::new();
        row.set_child(Some(&checkbox));
        list_box.append(&row);
    }

    let vbox = gtk4::Box::new(Orientation::Vertical, 5);

    vbox.set_margin_bottom(5);
    vbox.set_margin_top(5);
    vbox.set_margin_start(5);
    vbox.set_margin_end(5);

    vbox.append(&buttons_box);
    vbox.append(&list_box);
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Smart CPU")
        .default_width(400)
        .default_height(300)
        .child(&vbox)
        .resizable(true)
        .build();
    window.set_icon_name(Some("cpu"));
    window.show();
}