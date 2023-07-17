mod section;

use gtk::gdk;
use gtk::glib;
use gtk::graphene;
use gtk::gsk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use section::Section;

mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Default)]
    pub struct Wheel {
        sections: RefCell<Vec<Section>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Wheel {
        const NAME: &'static str = "OriWheel";
        type Type = super::Wheel;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for Wheel {
        fn constructed(&self) {
            self.parent_constructed();

            let sections = &mut self.sections.borrow_mut();
            let section = Section::new();

            section.set_min(0);
            section.set_max(59);

            section.set_parent(self.obj().as_ref());
            sections.push(section);
        }
    }

    impl WidgetImpl for Wheel {
        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            self.sections.borrow()[0].allocate(width, height, baseline, None);
        }

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

            let color = widget.color();

            snapshot.append_color(
                &color,
                &graphene::Rect::new(0.0, height * 0.5 - 20.0, width, 1.0),
            );
            snapshot.append_color(
                &color,
                &graphene::Rect::new(0.0, height * 0.5 + 20.0, width, 1.0),
            );
        }
    }
}

glib::wrapper! {
    pub struct Wheel(ObjectSubclass<imp::Wheel>)
        @extends gtk::Widget;
}

impl Wheel {}
