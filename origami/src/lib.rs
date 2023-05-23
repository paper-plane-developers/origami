//! [Paper Plane](https://github.com/paper-plane-developers/paper-plane) related set of gtk widgets that can be usable outside of it.

mod loading_indicator;
mod spoiler_overlay;

use gtk::prelude::StaticType;
pub use loading_indicator::LoadingIndicator;
pub use spoiler_overlay::SpoilerOverlay;

/// Registers all library types.
///
/// Expected to be called in the main function
pub fn init() {
    SpoilerOverlay::static_type();
    LoadingIndicator::static_type();
}
