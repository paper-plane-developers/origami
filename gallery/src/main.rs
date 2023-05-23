mod window;
use window::SpoilerWindow;

use adw::prelude::*;

fn main() {
    ori::init();

    let mut args = std::env::args();
    let name = args.next();
    let page = args.next();

    if args.next().is_some() {
        eprintln!("Too many arguments");
        std::process::exit(1);
    }

    let app = adw::Application::builder()
        .application_id("com.github.paper-plane-developers.OrigamiGallery")
        .build();

    app.connect_activate(move |app| SpoilerWindow::new(app, page.clone()).present());

    app.run_with_args(&[name.unwrap_or_default()]);
}
