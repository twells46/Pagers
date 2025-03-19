use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, glib};

mod lrsn;

const APP_ID: &str = "org.KIPR.Pagers";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Team 66")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|button| match lrsn::send_page("66") {
        Ok(_) => {
            println!("Success");
            button.set_label("Team 66 Paged!");
        }
        Err(e) => {
            eprintln!("Paging failed with: {}", e);
            button.set_label("Failed");
        }
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("KIPR Paging System")
        .child(&button)
        .build();

    window.present();
}
