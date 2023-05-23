use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/window/shimmer_effect/shimmer_effect.blp")]
    pub struct ShimmerEffectPage;

    #[glib::object_subclass]
    impl ObjectSubclass for ShimmerEffectPage {
        const NAME: &'static str = "OriDemoShimmerEffectPage";
        type Type = super::ShimmerEffectPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ShimmerEffectPage {}
    impl WidgetImpl for ShimmerEffectPage {}
    impl BinImpl for ShimmerEffectPage {}
}

glib::wrapper! {
    pub struct ShimmerEffectPage(ObjectSubclass<imp::ShimmerEffectPage>)
        @extends adw::Bin, gtk::Widget;
}
