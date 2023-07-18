use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk::gdk;
use gtk::gio;
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
        pub(super) children: [gtk::Button; 7],

        pub(super) active_shift: Cell<f64>,

        pub(super) snap_animation: OnceCell<adw::SpringAnimation>,

        pub(super) deceleration_animation: OnceCell<adw::SpringAnimation>,
        pub(super) deceleration_progress: Cell<f64>,

        #[property(get, set = Self::set_width_chars)]
        pub(super) width_chars: Cell<i32>,

        #[property(get, set = Self::set_selected)]
        pub(super) selected: Cell<i64>,

        #[property(get, set = Self::set_wrap)]
        pub(super) wrap: Cell<bool>,

        #[property(get, set)]
        pub(super) formatter: RefCell<Formatter>,

        #[property(get, set)]
        pub(super) model: RefCell<Option<gio::ListModel>>,

        #[property(get, set, nullable)]
        pub(super) expression: RefCell<Option<gtk::Expression>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Section {
        const NAME: &'static str = "OriWheelSection";
        type Type = super::Section;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("wheelsection");
            klass.set_accessible_role(gtk::AccessibleRole::List);
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

            widget.set_focusable(true);
            widget.set_can_focus(true);

            for (i, child) in self.children.iter().enumerate() {
                child.set_parent(widget);

                child.set_can_focus(false);
                child.set_focusable(false);

                child.add_css_class("flat");

                child.connect_clicked(clone!(@weak widget => move |_| {
                    let imp = widget.imp();

                    let i = imp.convert_position(i);

                    imp.pause_snap();

                    let shift = i as f64 + imp.active_shift.get();

                    let animation = imp.deceleration_animation.get().unwrap();

                    imp.deceleration_progress.take();

                    animation.set_initial_velocity(shift);
                    animation.set_value_to(shift);
                    animation.play();
                }));
            }

            let key_controller = gtk::EventControllerKey::new();

            key_controller.connect_key_pressed(move |controller, key, _, _| {
                let widget = controller.widget().downcast::<Self::Type>().unwrap();

                let shift = if key == gdk::Key::Up {
                    -1.0
                } else if key == gdk::Key::Down {
                    1.0
                } else {
                    return gtk::Inhibit(false);
                };

                let imp = widget.imp();

                let animation = imp.deceleration_animation.get().unwrap();

                imp.deceleration_progress.take();
                animation.set_value_to(shift);
                animation.play();

                gtk::Inhibit(true)
            });

            widget.add_controller(key_controller);

            let focus_controller = gtk::GestureClick::new();

            focus_controller.connect_released(move |controller, _, _, _| {
                let widget = controller.widget().downcast::<Self::Type>().unwrap();

                widget.grab_focus();
            });

            focus_controller.set_propagation_phase(gtk::PropagationPhase::Capture);

            widget.add_controller(focus_controller);

            let controller = gtk::EventControllerScroll::new(
                gtk::EventControllerScrollFlags::VERTICAL
                    | gtk::EventControllerScrollFlags::KINETIC,
            );

            controller.connect_scroll(move |controller, _x, y| {
                let widget = controller.widget().downcast::<Self::Type>().unwrap();
                widget.imp().handle_scroll(y * 0.1);
                gtk::Inhibit(true)
            });

            controller.connect_scroll_begin(move |controller| {
                let widget = controller.widget().downcast::<Self::Type>().unwrap();
                widget.imp().deceleration_animation.get().unwrap().pause();
                widget.imp().pause_snap();
            });

            controller.connect_scroll_end(|controller| {
                let widget = controller.widget().downcast::<Self::Type>().unwrap();
                widget.imp().animate_snap();
            });

            controller.connect_decelerate(move |controller, _, y| {
                let widget = controller.widget().downcast::<Self::Type>().unwrap();

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
            });

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

        fn measure(&self, orientation: gtk::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            if orientation == gtk::Orientation::Horizontal {
                let width = self.width_pixels();
                (width, width, -1, -1)
            } else {
                (-1, -1, -1, -1)
            }
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

                // I don't want items inside the selected area to be clickable
                child.set_can_target(i.abs() >= 0.8)
            }
        }
    }

    impl Section {
        fn handle_scroll(&self, y: f64) {
            let widget = self.obj();

            let imp = widget.imp();

            let new_shift = imp.active_shift.get() - y;

            let new_selected = imp.selected.get() - new_shift as i64;

            let count = imp
                .model
                .borrow()
                .as_ref()
                .map(|m| m.n_items())
                .unwrap_or_default() as i64;

            if self.wrap.get() {
                let selected = new_selected.rem_euclid(count);

                widget.set_selected(selected);

                imp.active_shift.set(new_shift % 1.0);
            } else {
                let max = count - 1;
                let selected = new_selected.max(0).min(max);

                widget.set_selected(selected);

                if selected == 0 && new_shift >= 0.0 || selected == max && new_shift <= 0.0 {
                    imp.active_shift.set(0.0);
                } else {
                    imp.active_shift.set(new_shift % 1.0);
                }
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

        fn set_wrap(&self, value: bool) {
            if self.wrap.replace(value) != value {
                self.refresh_inscriptions();
            }
        }

        fn set_width_chars(&self, value: i32) {
            if self.width_chars.replace(value) != value {
                self.obj().queue_resize();
            }
        }

        fn refresh_inscriptions(&self) {
            let value = self.selected.get();

            for (i, child) in self.children.iter().enumerate() {
                let i = self.convert_position(i);

                let index = i + value;

                if let Some(model) = &*self.model.borrow() {
                    let index = if self.wrap.get() {
                        index.rem_euclid(model.n_items() as i64)
                    } else {
                        index
                    };

                    if index < 0 {
                        child.set_visible(false);
                    } else {
                        if let Some(item) = model.item(index as u32) {
                            child.set_visible(true);

                            if let Some(expression) = &*self.expression.borrow() {
                                let label: String =
                                    expression.evaluate(Some(&item)).unwrap().get().unwrap();

                                child.set_label(&label);
                            } else {
                                if item.has_property("string", Some(glib::GString::static_type())) {
                                    let label: String = item.property("string");
                                    child.set_label(&label)
                                } else if let Some(label) =
                                    item.downcast_ref::<adw::EnumListItem>().map(|i| i.nick())
                                {
                                    child.set_label(&label)
                                } else {
                                    eprintln!("{} is unsupported by OriWheelSection. Custom expression is required", item.type_());
                                    child.set_visible(false);
                                }
                            }
                        } else {
                            child.set_visible(false);
                        }
                    }
                } else {
                    // child.set_visible(false);

                    child.set_label("Model is required");
                }
            }

            self.obj().queue_allocate();
            self.obj().queue_draw();
        }

        #[inline]
        fn convert_position(&self, index: usize) -> i64 {
            let child_count = self.children.len() as i64;
            let pos_shift = child_count / 2;
            (index as i64 - self.selected.get()).rem_euclid(child_count) - pos_shift
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

        fn width_pixels(&self) -> i32 {
            let metrics = self.obj().pango_context().metrics(None, None);

            let char_width = metrics
                .approximate_char_width()
                .max(metrics.approximate_digit_width());

            let width = char_width * self.width_chars.get();

            width / gtk::pango::SCALE
        }
    }
}

glib::wrapper! {
    pub struct Section(ObjectSubclass<imp::Section>)
        @extends gtk::Widget;
}

impl Section {
    pub fn new<M>(model: M, width_chars: i32) -> Self
    where
        M: IsA<gio::ListModel>,
    {
        let model: gio::ListModel = model.upcast();
        glib::Object::builder()
            .property("model", model)
            .property("width-chars", width_chars)
            .build()
    }
}
