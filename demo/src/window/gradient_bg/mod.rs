use adw::subclass::prelude::*;
use gtk::gdk;
use gtk::glib;

mod imp {
    use gtk::glib::clone;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/window/gradient_bg/gradient_bg.blp")]
    pub struct GradientBgPage {
        #[template_child]
        pub(super) drop_target: TemplateChild<gtk::DropTarget>,
        #[template_child]
        pub(super) gradient_bg: TemplateChild<ori::GradientBg>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GradientBgPage {
        const NAME: &'static str = "OriDemoGradientBgPage";
        type Type = super::GradientBgPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GradientBgPage {
        fn constructed(&self) {
            self.parent_constructed();

            let gradient_bg = self.gradient_bg.to_owned();

            let style_manager = adw::StyleManager::default();
            gradient_bg.set_dark(style_manager.is_dark());
            gradient_bg.set_theme_colors(hard_coded_theme_colors(style_manager.is_dark()));

            style_manager.connect_dark_notify(clone!(@weak gradient_bg => move |style_manager| {
                gradient_bg.set_dark(style_manager.is_dark());
                gradient_bg.set_theme_colors(hard_coded_theme_colors(style_manager.is_dark()))
            }));

            self.drop_target.connect_drop(
                clone!(@weak gradient_bg => @default-return false, move
                    |_, value, _, _ | {
                        let Ok(file_list) = value.get::<gdk::FileList>() else { return false; };

                        let pattern_texture = file_list.files().iter().filter_map(|file| gdk::Texture::from_file(file).ok()).next();
                        
                        gradient_bg.set_pattern(pattern_texture);
                        true
                    }
                ),
            );
        }
    }

    impl WidgetImpl for GradientBgPage {}
    impl BinImpl for GradientBgPage {}

    #[gtk::template_callbacks]
    impl GradientBgPage {
        #[template_callback]
        fn animate(&self) {
            self.gradient_bg.animate();
        }
    }
}

glib::wrapper! {
    pub struct GradientBgPage(ObjectSubclass<imp::GradientBgPage>)
        @extends adw::Bin, gtk::Widget;
}

fn hard_coded_theme_colors(dark: bool) -> Vec<gtk::graphene::Vec3> {
    let colors = if dark {
        vec![0xd6932e, 0xbc40db, 0x4280d7, 0x614ed5]
    } else {
        vec![0x94dae9, 0x9aeddb, 0x94c3f6, 0xac96f7]
    };

    let colors = colors
        .into_iter()
        .map(|int_color| {
            let r = (int_color >> 16) & 0xFF;
            let g = (int_color >> 8) & 0xFF;
            let b = int_color & 0xFF;

            gtk::graphene::Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
        })
        .collect();

    colors
}
