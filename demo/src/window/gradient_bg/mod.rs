use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk::gdk;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/window/gradient_bg/gradient_bg.blp")]
    pub struct GradientBgPage {
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
