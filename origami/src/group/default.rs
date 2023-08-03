use super::*;

use adw::prelude::*;
use gtk::glib;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;
    use std::cell::Cell;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = default::Group)]
    pub struct Group {
        #[property(get, set = Self::set_spacing)]
        pub(super) spacing: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Group {
        const NAME: &'static str = "OriGroup";
        type Type = default::Group;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("group");
        }
    }

    impl ObjectImpl for Group {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn dispose(&self) {
            self.obj().remove_children();
        }
    }

    impl WidgetImpl for Group {
        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            if for_size > 1 << 16 {
                return (-1, -1, -1, -1);
            }

            let widget = self.obj();

            if self.less_than_two_children() {
                return if let Some(child) = widget.first_child() {
                    child.measure(orientation, for_size)
                } else {
                    (0, 0, -1, -1)
                };
            }

            // TODO: just save aspect ratio and layout
            let layout = shared::layout(widget.as_ref().upcast_ref());

            let last_frame = layout.last().unwrap().layout_frame.get();

            let aspect_ratio = {
                let layout_width = last_frame.0 + last_frame.2;
                let layout_height = last_frame.1 + last_frame.3;
                layout_width / layout_height
            };

            if for_size == -1 {
                let size = if orientation == gtk::Orientation::Vertical {
                    // height
                    shared::TARGET_WIDTH / aspect_ratio
                } else {
                    shared::TARGET_WIDTH
                };
                return (0, size.round() as i32, -1, -1);
            };

            let size = if orientation == gtk::Orientation::Vertical {
                let width = for_size as f32;
                width / aspect_ratio
            } else {
                let heigth = for_size as f32;
                heigth * aspect_ratio
            }
            .round() as i32;

            if orientation == gtk::Orientation::Vertical {
                (size, size, -1, -1)
            } else {
                (0, size, -1, -1)
            }
        }

        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            let widget = self.obj();

            if self.less_than_two_children() {
                if let Some(child) = widget.first_child() {
                    child.allocate(width, height, baseline, None);
                }
            } else {
                let layout = shared::layout(widget.as_ref().upcast_ref());

                let scale = width as f32 / 480.0;
                let spacing = self.spacing.get() as f32;

                for (widget, cw) in widget.iter_children().zip(layout.iter()) {
                    cw.apply(&widget, scale, spacing);
                }
            }
        }
    }

    impl Group {
        fn set_spacing(&self, spacing: i32) {
            self.spacing.set(spacing);
            self.obj().queue_allocate();
        }

        fn less_than_two_children(&self) -> bool {
            if let Some(first) = self.obj().first_child() {
                first.next_sibling().is_none()
            } else {
                true
            }
        }
    }
}

glib::wrapper! {
    pub struct Group(ObjectSubclass<imp::Group>)
        @extends gtk::Widget;
}

impl Group {
    pub fn remove_children(&self) {
        while let Some(child) = self.first_child() {
            child.unparent();
        }
    }

    pub fn append(&self, child: &impl IsA<gtk::Widget>) {
        child.insert_before(self, gtk::Widget::NONE);
    }

    pub fn replace_children<I, W>(&self, children: I)
    where
        I: IntoIterator<Item = W>,
        W: IsA<gtk::Widget>,
    {
        self.remove_children();
        for child in children {
            self.append(&child);
        }
    }
}
