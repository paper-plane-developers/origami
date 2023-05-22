mod spoiler_overlay;

use gtk::prelude::StaticType;
pub use spoiler_overlay::SpoilerOverlay;

pub fn init() {
    SpoilerOverlay::static_type();
}
