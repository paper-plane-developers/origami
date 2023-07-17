mod formatter;
mod section;

use glib::clone;
use gtk::gdk;
use gtk::glib;
use gtk::graphene;
use gtk::gsk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use section::Section;

pub use formatter::Formatter;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct Wheel {}

    #[glib::object_subclass]
    impl ObjectSubclass for Wheel {
        const NAME: &'static str = "OriWheel";
        type Type = super::Wheel;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for Wheel {
        fn constructed(&self) {
            self.parent_constructed();

            let widget = &*self.obj();

            widget.set_layout_manager(Some(gtk::BoxLayout::new(gtk::Orientation::Horizontal)));

            widget.set_hexpand(true);

            widget.set_halign(gtk::Align::Center);

            let year_section = Section::new();

            year_section.set_min(1960);
            year_section.set_max(2060);
            year_section.set_selected(2023);

            year_section.set_width_request(80);

            year_section.set_parent(widget);

            let month_section = Section::new();

            month_section.set_min(0);
            month_section.set_max(11);

            month_section.set_width_request(120);

            let month_formatter = Formatter::new(move |index| {
                [
                    "January",
                    "February",
                    "March",
                    "April",
                    "May",
                    "June",
                    "July",
                    "August",
                    "September",
                    "October",
                    "November",
                    "December",
                ][index.rem_euclid(12) as usize]
                    .to_owned()
            });

            month_section.set_formatter(month_formatter);

            month_section.set_parent(widget);

            let day_section = Section::new();

            day_section.set_min(1);
            day_section.set_max(31);

            day_section.set_width_request(60);

            day_section.set_parent(widget);

            let day_updater = clone!(@weak year_section, @weak month_section, @weak day_section => move |_: &Section| {
                let year = year_section.selected();
                let month = month_section.selected();

                let month = glib::DateMonth::__Unknown(month as i32 + 1);

                let day_count = glib::Date::days_in_month(month, year as u16);

                day_section.set_max(day_count as i64);
            });

            month_section.connect_selected_notify(day_updater.clone());
            year_section.connect_selected_notify(day_updater.clone());
        }
    }

    impl WidgetImpl for Wheel {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let widget = self.obj();

            let width = widget.width() as f32;
            let height = widget.height() as f32;

            let wheel_height = 200.0;

            let clip_bounds =
                &graphene::Rect::new(0.0, (height - wheel_height) * 0.5, width, wheel_height);

            snapshot.push_mask(gsk::MaskMode::Alpha);

            snapshot.append_linear_gradient(
                &clip_bounds,
                &clip_bounds.top_left(),
                &clip_bounds.bottom_left(),
                &[
                    gsk::ColorStop::new(0.0, gdk::RGBA::TRANSPARENT),
                    gsk::ColorStop::new(0.1, gdk::RGBA::WHITE),
                    gsk::ColorStop::new(0.9, gdk::RGBA::WHITE),
                    gsk::ColorStop::new(1.0, gdk::RGBA::TRANSPARENT),
                ],
            );

            snapshot.pop(); // mask 1

            self.parent_snapshot(snapshot);

            snapshot.pop(); // mask 2

            let mut color = widget.color();

            color.set_alpha(0.1);

            let selection_bounds = graphene::Rect::new(0.0, height * 0.5 - 20.0, width, 40.0);

            snapshot.push_rounded_clip(&gsk::RoundedRect::from_rect(selection_bounds, 8.0));

            snapshot.append_color(&color, &selection_bounds);

            snapshot.pop();
        }
    }
}

glib::wrapper! {
    pub struct Wheel(ObjectSubclass<imp::Wheel>)
        @extends gtk::Widget;
}

impl Wheel {}
