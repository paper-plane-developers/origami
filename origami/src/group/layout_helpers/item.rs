use super::position_flags::PositionFlags;
use std::cell::Cell;

use gtk::gsk;
use gtk::{graphene, prelude::*};

use crate::traits::Lerp;

#[derive(Debug, Clone)]
pub(crate) struct LayoutItem {
    pub(crate) aspect_ratio: f32,
    pub(crate) layout_frame: Cell<(f32, f32, f32, f32)>,
    pub(crate) position_flags: Cell<PositionFlags>,
}

impl LayoutItem {
    pub fn new(widget: gtk::Widget) -> Self {
        let aspect_ratio = if widget.has_property("aspect-ratio", Some(f32::static_type())) {
            // get rid of the warning
            _ = widget.measure(gtk::Orientation::Vertical, -1);
            widget.property("aspect-ratio")
        } else {
            let (_min, natural) = widget.preferred_size();

            natural.width().max(1) as f32 / natural.height().max(1) as f32
        };

        Self {
            aspect_ratio,
            layout_frame: Cell::default(),
            position_flags: Default::default(),
        }
    }

    pub fn apply(&self, widget: &gtk::Widget, scale: f32, spacing: f32) {
        let (mut shift_x, mut shift_y, mut width, mut height) = self.layout_frame.get();

        // Apply scale
        shift_x *= scale;
        shift_y *= scale;
        width *= scale;
        height *= scale;

        // Apply spacing
        let pos_flags = self.position_flags.get();

        let half_spacing = spacing * 0.5;

        if !pos_flags.at_left() {
            shift_x += half_spacing;
            width -= half_spacing;
        }
        if !pos_flags.at_top() {
            shift_y += half_spacing;
            height -= half_spacing;
        }
        if !pos_flags.at_right() {
            width -= half_spacing;
        }
        if !pos_flags.at_bottom() {
            height -= half_spacing;
        }

        // Otherwise values would be rounded wrong way
        shift_x = shift_x.round();
        shift_y = shift_y.round();
        width = width.round();
        height = height.round();

        // Remove classes
        for class in ["left", "top", "right", "bottom"] {
            widget.remove_css_class(class);
        }

        let mut classes = [None; 4];

        if pos_flags.at_left() {
            classes[0] = Some("left");
        }
        if pos_flags.at_top() {
            classes[1] = Some("top");
        }
        if pos_flags.at_right() {
            classes[2] = Some("right");
        }
        if pos_flags.at_bottom() {
            classes[3] = Some("bottom");
        }

        for class in classes.into_iter().flatten() {
            widget.add_css_class(class);
        }

        // Allocate widget
        let transform = gsk::Transform::new().translate(&graphene::Point::new(shift_x, shift_y));

        widget.allocate(width as i32, height as i32, -1, Some(transform))
    }
}

impl Lerp<f32> for &LayoutItem {
    type Output = LayoutItem;

    fn lerp(self, other: &LayoutItem, t: f32) -> LayoutItem {
        let start_frame = self.layout_frame.get();
        let end_frame = other.layout_frame.get();

        let frame = start_frame.lerp(end_frame, t);

        let res = if t < 0.5 { self.clone() } else { other.clone() };

        res.layout_frame.set(frame);
        res
    }
}

impl Lerp<f32> for &[LayoutItem] {
    type Output = Vec<LayoutItem>;

    fn lerp(self, other: Self, t: f32) -> Self::Output {
        self.iter()
            .zip(other.iter())
            .map(|(s, e)| s.lerp(e, t))
            .collect()
    }
}
