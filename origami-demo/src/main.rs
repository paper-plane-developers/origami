mod config {
    #![allow(dead_code)]

    include!(concat!(env!("CODEGEN_BUILD_DIR"), "/config.rs"));
}

mod application;
mod window;

use gettextrs::{gettext, LocaleCategory};
use gtk::glib;

use self::application::ExampleApplication;
use self::config::{GETTEXT_PACKAGE, LOCALEDIR};

fn main() -> glib::ExitCode {
    adw::init().unwrap();
    ori::init();

    // Initialize logger
    tracing_subscriber::fmt::init();

    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext("Origami Demo"));

    let app = ExampleApplication::default();
    app.run()
}
