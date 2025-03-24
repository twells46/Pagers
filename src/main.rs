use glib::clone;
use gtk::{Application, ApplicationWindow, Button, Grid, ScrolledWindow, gio, glib};
use gtk::{Label, prelude::*};
use std::process;
use std::thread;
use std::time::Duration;

mod csvproc;
use csvproc::{Team, read_teams};

mod lrsn;

const APP_ID: &str = "org.KIPR.Pagers";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn labeler(label: &str) -> Label {
    Label::builder()
        .label(label)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .margin_top(12)
        .build()
}

fn append_to_grid(row_number: i32, team: Team, grid: &Grid) {
    let team_num = labeler(&team.num.to_string());
    let school = labeler(&team.school);

    let button = Button::builder()
        .label(format!("Page team {}", team.num))
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let (sender, receiver) = async_channel::bounded(1);

    button.connect_clicked(move |button| {
        let sender = sender.clone();
        match lrsn::send_page(team.num) {
            Ok(_) => {
                println!("Success");
                button.set_label(&format!("Team {} paged!", team.num));
            }
            Err(e) => {
                eprintln!("Paging failed with: {}", e);
                button.set_label("Failed");
            }
        }
        gio::spawn_blocking(move || {
            let sender = sender.clone();
            sender.send_blocking(false).expect("Channel not open");
            thread::sleep(Duration::from_secs(10));
            sender.send_blocking(true).expect("Channel not open");
        });
    });

    glib::spawn_future_local(clone!(
        #[weak]
        button,
        async move {
            while let Ok(enable_button) = receiver.recv().await {
                button.set_sensitive(enable_button);
                if enable_button {
                    button.set_label(&format!("Page team {}", team.num));
                }
            }
        }
    ));

    grid.attach(&team_num, 0, row_number, 1, 1);
    grid.attach(&school, 2, row_number, 1, 1);
    grid.attach(&button, 3, row_number, 1, 1);
}

fn build_ui(app: &Application) {
    let mut teams = match read_teams("Oklahoma") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to read csv file: {}", e);
            process::exit(1)
        }
    };

    let grid = Grid::new();

    let mut i = 0;
    while let Some(team) = teams.pop() {
        append_to_grid(i, team, &grid);
        i += 1;
    }

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .child(&grid)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("KIPR Paging System")
        .default_width(400)
        .default_height(600)
        .child(&scrolled_window)
        .build();

    window.present();
}
