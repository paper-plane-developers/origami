use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk::glib;
use gtk::graphene;
use gtk::gsk;
use std::cell::Cell;

const GRADIENT_WIDTH: f32 = 256.0;
const GRADIENT_PIXELS_PER_SEC: f32 = 100.0;

mod imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::ShimmerEffect)]
    pub struct ShimmerEffect {
        #[property(get, set)]
        pub(super) playing: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ShimmerEffect {
        const NAME: &'static str = "OriShimmerEffect";
        type Type = super::ShimmerEffect;
        type ParentType = adw::Bin;
    }

    impl ObjectImpl for ShimmerEffect {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }

        fn constructed(&self) {
            self.parent_constructed();
            self.obj().connect_child_notify(|obj| {
                if let Some(child) = obj.child() {
                    child.connect_visible_notify(clone!(@weak obj => move |child| {
                        if child.is_visible() {
                            obj.queue_draw();
                        }
                    }));
                }
            });

            self.obj().connect_playing_notify(|widget| {
                if widget.playing() {
                    widget.add_tick_callback(|widget, _clock| {
                        widget.queue_draw();
                        glib::ControlFlow::from(widget.playing())
                    });
                }
            });
        }
    }

    impl WidgetImpl for ShimmerEffect {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            if !self.playing.get() {
                self.parent_snapshot(snapshot);
            }

            let widget = self.obj();

            snapshot.push_mask(gsk::MaskMode::Alpha);
            self.parent_snapshot(snapshot);
            snapshot.pop();

            let window = self
                .obj()
                .ancestor(gtk::Window::static_type())
                .and_downcast::<gtk::Window>()
                .unwrap();

            let win_bounds = window.compute_bounds(self.obj().as_ref()).unwrap();

            let time_secs = std::time::Duration::from_micros(widget.time() as u64).as_secs_f32();

            let shift = time_secs * GRADIENT_PIXELS_PER_SEC % GRADIENT_WIDTH;

            let gradient_bounds =
                graphene::Rect::new(win_bounds.x() + shift, win_bounds.y(), GRADIENT_WIDTH, 1.0);

            let mut color1 = widget.color();
            let mut color2 = color1.clone();
            color1.set_alpha(0.6);
            color2.set_alpha(0.3);

            snapshot.append_repeating_linear_gradient(
                &win_bounds,
                &gradient_bounds.top_left(),
                &gradient_bounds.top_right(),
                &[
                    gsk::ColorStop::new(0.0, color2),
                    gsk::ColorStop::new(0.4, color1),
                    gsk::ColorStop::new(0.6, color1),
                    gsk::ColorStop::new(1.0, color2),
                ],
            );

            snapshot.pop();
        }
    }
    impl BinImpl for ShimmerEffect {}
}

glib::wrapper! {
    /// Pulsating shimmer effect
    ///
    /// Useful for skeleton loaders
    ///
    /// # Properties
    /// * playing: [bool].
    /// Controls whether to display the effect
    pub struct ShimmerEffect(ObjectSubclass<imp::ShimmerEffect>)
        @extends adw::Bin, gtk::Widget;
}

impl ShimmerEffect {
    fn time(&self) -> i64 {
        self.frame_clock()
            .and_then(|clk| clk.current_timings())
            .map(|t| t.frame_time())
            .unwrap_or_default()
    }
}
