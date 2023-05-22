mod window;
use window::SpoilerWindow;

use adw::prelude::*;

fn main() {
    ori::init();

    let app = adw::Application::builder()
        .application_id("com.github.paper-plane-developers.OrigamiGallery")
        .build();

    app.connect_activate(move |app| SpoilerWindow::new(app).present());

    app.run();
}
