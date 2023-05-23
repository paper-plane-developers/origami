mod loading_indicator;
mod spoiler_overlay;

use gtk::prelude::StaticType;
pub use loading_indicator::LoadingIndicator;
pub use spoiler_overlay::SpoilerOverlay;

pub fn init() {
    SpoilerOverlay::static_type();
    LoadingIndicator::static_type();
}
