//! [Paper Plane](https://github.com/paper-plane-developers/paper-plane) related set of gtk widgets that can be usable outside of it.

mod loading_indicator;
mod shimmer_effect;
mod spoiler_overlay;
mod wheel;

use gtk::prelude::StaticType;
pub use loading_indicator::LoadingIndicator;
pub use shimmer_effect::ShimmerEffect;
pub use spoiler_overlay::SpoilerOverlay;
pub use wheel::Wheel;

/// Registers all library types. And inits stylesheet
///
/// Expected to be called in the main function
pub fn init() {
    adw::init().unwrap();
    init_style_sheet();

    LoadingIndicator::static_type();
    ShimmerEffect::static_type();
    SpoilerOverlay::static_type();
    Wheel::static_type();
}

fn init_style_sheet() {
    let provider = gtk::CssProvider::new();

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &provider,
        100,
    );

    let style_manager = adw::StyleManager::default();

    let refresh_style = move |style_manager: &adw::StyleManager| {
        if style_manager.is_dark() {
            provider.load_from_data(concat!(
                include_str!("./styles.css"),
                include_str!("./styles-dark.css")
            ));
        } else {
            provider.load_from_data(include_str!("./styles.css"));
        }
    };

    refresh_style(&style_manager);
    style_manager.connect_dark_notify(refresh_style);
}
