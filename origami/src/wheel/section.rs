use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk::glib;
use gtk::graphene;
use gtk::gsk;

mod imp {
    use std::cell::{Cell, OnceCell};

    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::Section)]
    pub struct Section {
        inscriptions: [gtk::Inscription; 5],

        #[property(get, set)]
        pub(super) min: Cell<i32>,

        #[property(get, set)]
        pub(super) max: Cell<i32>,

        #[property(get, set = Self::set_selected)]
        pub(super) selected: Cell<i32>,

        pub(super) active_shift: Cell<f64>,

        pub(super) animation: OnceCell<adw::SpringAnimation>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Section {
        const NAME: &'static str = "OriWheelSection";
        type Type = super::Section;
        type ParentType = gtk::Widget;
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

            for inscription in &self.inscriptions {
                inscription.set_parent(widget);
                inscription.set_xalign(0.5);
                inscription.set_yalign(0.5);
            }

            let controller = gtk::EventControllerScroll::new(
                gtk::EventControllerScrollFlags::VERTICAL
                    | gtk::EventControllerScrollFlags::KINETIC,
            );

            controller.connect_scroll(clone!(@weak widget => @default-return gtk::Inhibit(false),
                move  |_, _x, y| {

                    let imp = widget.imp();

                    let new_shift = imp.active_shift.get() - y * 0.1;

                    let new_selected = imp.selected.get() - new_shift as i32;

                    let min = imp.min.get();
                    let max = imp.max.get();

                    let selected = new_selected.max(min).min(max);

                    widget.set_selected(selected);

                    if selected == min && new_shift >= 0.0 || selected == max && new_shift <= 0.0{
                        imp.active_shift.set(0.0);
                    } else {
                        imp.active_shift.set(new_shift % 1.0);
                    }

                    widget.queue_draw();

                    gtk::Inhibit(true)
            }));

            controller.connect_scroll_begin(clone!(@weak widget => move |_| {
                widget.imp().animation.get().unwrap().pause();
            }));

            controller.connect_scroll_end(clone!(@weak widget => move |_| {
                let imp = widget.imp();

                let animation = imp.animation.get().unwrap();
                animation.set_value_from(imp.active_shift.get());

                animation.play();
            }));

            controller.connect_decelerate(clone!(@weak widget => move |_, _, _y| {
                // TODO: implement kinetic scrolling

                // dbg!(y);

                // let imp = widget.imp();

                // let animation = imp.animation.get().unwrap();
                // animation.set_value_from(imp.active_shift.get());

                // animation.play();
            }));

            widget.add_controller(controller);

            let target = adw::CallbackAnimationTarget::new(clone!(@weak widget => move |val| {
                let imp = widget.imp();
                imp.active_shift.set(val);
                widget.queue_draw();
            }));
            let params = adw::SpringParams::new(1.00, 1.0, 100.0);
            let animation = adw::SpringAnimation::new(widget, 0.0, 0.0, params, target);

            self.animation.set(animation).unwrap();
        }
    }

    impl WidgetImpl for Section {
        fn realize(&self) {
            self.parent_realize();
            self.refresh_inscriptions();
        }

        fn size_allocate(&self, width: i32, _height: i32, _baseline: i32) {
            for inscription in &self.inscriptions {
                let (_, size) = inscription.preferred_size();

                let transform = gsk::Transform::new().translate(&graphene::Point::new(
                    -width as f32 * 0.5,
                    -size.height() as f32 * 0.5,
                ));

                inscription.allocate(width, size.height(), -1, Some(transform));
            }
        }

        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let widget = self.obj();

            let width = widget.width() as f32;
            let height = widget.height() as f32;

            let bounds = graphene::Rect::new(0.0, 0.0, width, height);

            let center = bounds.center();

            snapshot.translate(&center);

            fn transform_for_position(position: f64) -> gsk::Transform {
                let position = position as f32;

                gsk::Transform::new().translate(&graphene::Point::new(0.0, position * 50.0))
            }

            let shift = self.selected.get();

            for (i, child) in self.inscriptions.iter().enumerate() {
                let i = (i as i32 + shift).rem_euclid(5) - 2;

                let i = i as f64 + self.active_shift.get();

                let transform = transform_for_position(i);

                snapshot.transform(Some(&transform));

                widget.snapshot_child(child, snapshot);

                snapshot.transform(Some(&transform.invert().unwrap()));
            }
        }
    }

    impl Section {
        fn set_selected(&self, value: i32) {
            if self.selected.get() == value {
                return;
            }

            self.selected.set(value);

            self.refresh_inscriptions();
        }

        fn refresh_inscriptions(&self) {
            let value = self.selected.get();

            for (i, child) in self.inscriptions.iter().enumerate() {
                let i = (i as i32 + value).rem_euclid(5) - 2;

                let index = i + value;

                if index < self.min.get() || index > self.max.get() {
                    child.set_text(None);
                } else {
                    child.set_text(Some(&(index).to_string()));
                }
            }

            self.obj().queue_draw();
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
