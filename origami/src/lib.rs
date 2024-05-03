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

/// Registers all library types.
///
/// Expected to be called in the main function
pub fn init() {
    LoadingIndicator::static_type();
    ShimmerEffect::static_type();
    SpoilerOverlay::static_type();
    Wheel::static_type();
}
