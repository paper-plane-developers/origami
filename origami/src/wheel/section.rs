use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk::glib;
use gtk::graphene;
use gtk::gsk;

use super::Formatter;

mod imp {
    use std::cell::{Cell, OnceCell, RefCell};

    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::Section)]
    pub struct Section {
        children: [gtk::Button; 5],

        #[property(get, set = Self::set_min)]
        pub(super) min: Cell<i64>,

        #[property(get, set = Self::set_max)]
        pub(super) max: Cell<i64>,

        #[property(get, set = Self::set_selected)]
        pub(super) selected: Cell<i64>,

        pub(super) active_shift: Cell<f64>,

        pub(super) snap_animation: OnceCell<adw::SpringAnimation>,

        pub(super) deceleration_animation: OnceCell<adw::SpringAnimation>,
        pub(super) deceleration_progress: Cell<f64>,

        #[property(get, set)]
        pub(super) formatter: RefCell<Formatter>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Section {
        const NAME: &'static str = "OriWheelSection";
        type Type = super::Section;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("wheelsection");
        }
    }

    impl ObjectImpl for Section {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            Self::derived_set_property(&self, id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            Self::derived_property(&self, id, pspec)
        }

        fn constructed(&self) {
            self.parent_constructed();

            let widget = &*self.obj();

            for (i, child) in self.children.iter().enumerate() {
                child.set_parent(widget);

                child.add_css_class("flat");

                child.connect_clicked(clone!(@weak widget => move |_| {
                    let imp = widget.imp();

                    let i = imp.convert_position(i);

                    imp.pause_snap();

                    let shift = i as f64 + imp.active_shift.get();

                    let animation = imp.deceleration_animation.get().unwrap();

                    imp.deceleration_progress.take();
                    animation.set_value_to(shift);
                    animation.play();
                }));
            }

            let controller = gtk::EventControllerScroll::new(
                gtk::EventControllerScrollFlags::VERTICAL
                    | gtk::EventControllerScrollFlags::KINETIC,
            );

            controller.connect_scroll(clone!(@weak widget => @default-return gtk::Inhibit(false),
                move  |_, _x, y| {
                    widget.imp().handle_scroll(y * 0.1);
                    gtk::Inhibit(true)
            }));

            controller.connect_scroll_begin(clone!(@weak widget => move |_| {
                widget.imp().deceleration_animation.get().unwrap().pause();
                widget.imp().pause_snap();
            }));

            controller.connect_scroll_end(clone!(@weak widget => move |_| {
                widget.imp().animate_snap();
            }));

            controller.connect_decelerate(clone!(@weak widget => move |_, _, y| {
                if y.abs() <= f64::EPSILON {
                    return;
                }

                let imp = widget.imp();

                imp.deceleration_progress.set(0.0);

                imp.pause_snap();

                let animation = imp.deceleration_animation.get().unwrap();

                animation.set_initial_velocity(y * 0.001);
                animation.set_value_to(y * 0.01);

                animation.play();
            }));

            widget.add_controller(controller);

            let target = adw::CallbackAnimationTarget::new(clone!(@weak widget => move |val| {
                let imp = widget.imp();
                imp.active_shift.set(val);
                widget.queue_allocate();
                widget.queue_draw();
            }));
            let params = adw::SpringParams::new(1.00, 1.0, 100.0);
            let animation = adw::SpringAnimation::new(widget, 0.0, 0.0, params, target);

            self.snap_animation.set(animation).unwrap();

            let target = adw::CallbackAnimationTarget::new(clone!(@weak widget => move |val| {
                let imp = widget.imp();
                let shift = val - imp.deceleration_progress.replace(val);
                imp.handle_scroll(shift);
            }));

            let params = adw::SpringParams::new(1.00, 0.1, 100.0);
            let animation = adw::SpringAnimation::new(widget, 0.0, 1.0, params, target);

            animation.set_clamp(true);

            animation.connect_done(clone!(@weak widget => move |_| {
                widget.imp().deceleration_progress.take();
                widget.imp().animate_snap();
            }));

            self.deceleration_animation.set(animation).unwrap();
        }
    }

    impl WidgetImpl for Section {
        fn realize(&self) {
            self.parent_realize();
            self.refresh_inscriptions();
        }

        fn size_allocate(&self, width: i32, height: i32, _baseline: i32) {
            fn transform_for_position(position: f64) -> gsk::Transform {
                let position = position as f32;

                gsk::Transform::new().translate(&graphene::Point::new(0.0, position * 50.0))
            }

            for (i, child) in self.children.iter().enumerate() {
                let i = self.convert_position(i);

                let i = i as f64 + self.active_shift.get();

                let (_, size) = child.preferred_size();

                let transform = transform_for_position(i).translate(&graphene::Point::new(
                    0.0,
                    height as f32 * 0.5 - size.height() as f32 * 0.5,
                ));

                child.allocate(width, size.height(), -1, Some(transform));
            }
        }
    }

    impl Section {
        fn handle_scroll(&self, y: f64) {
            let widget = self.obj();

            let imp = widget.imp();

            let new_shift = imp.active_shift.get() - y;

            let new_selected = imp.selected.get() - new_shift as i64;

            let min = imp.min.get();
            let max = imp.max.get();

            let selected = new_selected.max(min).min(max);

            widget.set_selected(selected);

            if selected == min && new_shift >= 0.0 || selected == max && new_shift <= 0.0 {
                imp.active_shift.set(0.0);
            } else {
                imp.active_shift.set(new_shift % 1.0);
            }

            widget.queue_allocate();
            widget.queue_draw();
        }

        fn set_selected(&self, value: i64) {
            if self.selected.get() == value {
                return;
            }

            self.selected.set(value);

            self.refresh_inscriptions();
        }

        fn set_min(&self, value: i64) {
            self.min.set(value);

            if self.selected.get() < value {
                self.obj().set_selected(value);
            } else {
                self.refresh_inscriptions();
            }
        }

        fn set_max(&self, value: i64) {
            self.max.set(value);

            if self.selected.get() > value {
                self.obj().set_selected(value);
            } else {
                self.refresh_inscriptions();
            }
        }

        fn refresh_inscriptions(&self) {
            let value = self.selected.get();

            for (i, child) in self.children.iter().enumerate() {
                let child_index = i;

                let i = self.convert_position(i);

                let index = i + value;

                if index < self.min.get() || index > self.max.get() {
                    child.set_visible(false);
                } else {
                    let string = self.formatter.borrow().format(index);
                    child.set_visible(true);

                    child.set_label(&format!("{child_index}: {string}"));
                }
            }

            self.obj().queue_allocate();
            self.obj().queue_draw();
        }

        #[inline]
        fn convert_position(&self, index: usize) -> i64 {
            (index as i64 - self.selected.get()).rem_euclid(5) - 2
        }

        fn pause_snap(&self) {
            self.snap_animation.get().unwrap().pause();
        }

        fn animate_snap(&self) {
            let animation = self.snap_animation.get().unwrap();
            let shift = self.active_shift.get();

            let from = shift - shift.round();

            self.active_shift.set(from);
            self.set_selected(self.selected.get() - shift.round() as i64);

            animation.set_value_from(from);
            animation.play();
        }
    }
}

glib::wrapper! {
    pub struct Section(ObjectSubclass<imp::Section>)
        @extends gtk::Widget;
}

impl Section {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
