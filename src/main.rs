use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

const APP_ID: &str = "org.friendlyhub.DummyApp";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::with_label("Click me");
    button.connect_clicked(|btn| {
        btn.set_label("Hello World!");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dummy App")
        .default_width(300)
        .default_height(200)
        .child(&button)
        .build();

    window.present();
}
