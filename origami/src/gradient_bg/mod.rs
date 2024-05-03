mod software_gradient;

use std::cell::Cell;
use std::cell::OnceCell;
use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk::gdk;
use gtk::glib;
use gtk::graphene;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct GradientBg {
        pub(super) gradient_texture: RefCell<Option<gdk::MemoryTexture>>,

        pub(super) pattern: RefCell<Option<gdk::Texture>>,

        pub(super) animation: OnceCell<adw::Animation>,
        pub(super) progress: Cell<f32>,
        pub(super) phase: Cell<u32>,

        pub(super) dark: Cell<bool>,
        pub(super) colors: RefCell<Vec<software_gradient::Color>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GradientBg {
        const NAME: &'static str = "OriGradientBg";
        type Type = super::GradientBg;
        type ParentType = adw::Bin;
    }

    impl ObjectImpl for GradientBg {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let style_manager = adw::StyleManager::default();

            if style_manager.is_high_contrast() {
                obj.add_css_class("fallback");
            }

            style_manager.connect_high_contrast_notify(clone!(@weak obj => move |style_manager| {
                if style_manager.is_high_contrast() {
                    obj.add_css_class("fallback");
                }
            }));

            let target = adw::CallbackAnimationTarget::new(clone!(@weak obj => move |progress| {
                let imp = obj.imp();
                imp.gradient_texture.take();
                let progress = progress as f32;
                if progress >= 1.0 {
                    imp.progress.set(0.0);
                    imp.phase.set((imp.phase.get() + 1) % 8);
                } else {
                    imp.progress.set(progress)
                }
                obj.queue_draw();
            }));

            let animation = adw::TimedAnimation::builder()
                .widget(&*obj)
                .value_from(0.0)
                .value_to(1.0)
                .duration(200)
                .target(&target)
                .easing(adw::Easing::EaseInOutQuad)
                .build()
                .upcast();

            self.animation.set(animation).unwrap();
        }
    }

    impl WidgetImpl for GradientBg {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let widget = self.obj();

            if widget.has_css_class("fallback") {
                // fallback code
                if let Some(child) = widget.child() {
                    widget.snapshot_child(&child, snapshot);
                }
                return;
            };

            let width = widget.width() as f32;
            let height = widget.height() as f32;

            if width == 0.0 || height == 0.0 {
                return;
            }

            let bounds = graphene::Rect::new(0.0, 0.0, width, height);

            self.snapshot_gradient(snapshot, &bounds);

            self.snapshot_pattern(snapshot, &bounds);
        }
    }

    impl BinImpl for GradientBg {}

    impl GradientBg {
        fn snapshot_gradient(&self, snapshot: &gtk::Snapshot, bounds: &graphene::Rect) {
            let progress = self.progress.get();
            let phase = self.phase.get() as usize;

            let cached_texture = (*self.gradient_texture.borrow()).clone();

            let texture = if let Some(texture) = cached_texture {
                texture
            } else {
                let colors = &*self.colors.borrow();

                let positions = Self::calculate_positions(progress, phase);

                // Even with 4x4 the upscaled result looks good,
                // but I think that 32x32 is better
                let (width, height) = (32, 32);

                let raw_texture =
                    software_gradient::generate_gradient(width, height, colors, &positions);

                let texture = gdk::MemoryTexture::new(
                    width as i32,
                    height as i32,
                    gdk::MemoryFormat::B8g8r8a8,
                    &glib::Bytes::from_owned(raw_texture),
                    4 * width as usize,
                );

                if progress == 0.0 {
                    self.gradient_texture.replace(Some(texture.clone()));
                }

                texture
            };

            snapshot.append_texture(&texture, &bounds);
        }

        fn snapshot_pattern(&self, snapshot: &gtk::Snapshot, bounds: &graphene::Rect) {
            let widget = self.obj();
            let Some(pattern) = &*self.pattern.borrow() else {
                // Nothing to snapshot
                return;
            };

            let pattern_bounds = graphene::Rect::new(
                0.0,
                0.0,
                pattern.width() as f32 * 0.3,
                pattern.height() as f32 * 0.3,
            );

            let mut matrix = [0.0; 16];
            let mut offset = [0.0; 4];
            if self.dark.get() {
                matrix[15] = -0.3;
                offset = [0.08; 4];
                offset[3] = 1.0;
            } else {
                matrix[15] = 0.1;
            }
            let color_matrix = graphene::Matrix::from_float(matrix);
            let color_offset = graphene::Vec4::from_float(offset);

            snapshot.push_color_matrix(&color_matrix, &color_offset);
            snapshot.push_repeat(bounds, Some(&pattern_bounds));
            snapshot.append_texture(pattern, &pattern_bounds);
            snapshot.pop();
            snapshot.pop();

            if let Some(child) = widget.child() {
                widget.snapshot_child(&child, snapshot);
            }
        }

        fn calculate_positions(progress: f32, phase: usize) -> Vec<software_gradient::Point> {
            let mut current = software_gradient::gather_positions(phase);
            let next = software_gradient::gather_positions(phase + 1);

            if progress > 0.0 {
                current
                    .iter_mut()
                    .zip(next)
                    .for_each(|(current, next)| *current = current.interpolate(next, progress))
            }

            current
        }
    }
}

glib::wrapper! {
    pub struct GradientBg(ObjectSubclass<imp::GradientBg>)
        @extends gtk::Widget, adw::Bin;
}

impl GradientBg {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_pattern(&self, pattern: Option<gdk::Texture>) {
        self.imp().pattern.replace(pattern);
    }

    pub fn set_dark(&self, dark: bool) {
        self.imp().dark.set(dark);
    }

    // Takes int colors as from theme returned by tdlib
    pub fn set_theme_colors(&self, colors: &[i32]) {
        let imp = self.imp();

        let colors = colors
            .iter()
            .map(|&int| software_gradient::Color::from_int_rgb(int))
            .collect();

        imp.colors.replace(colors);

        imp.gradient_texture.take();
        self.queue_draw();
    }

    pub fn animate(&self) {
        let animation = self.imp().animation.get().unwrap();

        let val = animation.value();
        if val == 0.0 || val == 1.0 {
            animation.play()
        }
    }
}

impl Default for GradientBg {
    fn default() -> Self {
        Self::new()
    }
}
