use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/window/loading_indicator/loading_indicator.blp")]
    pub struct LoadingIndicatorPage {}

    #[glib::object_subclass]
    impl ObjectSubclass for LoadingIndicatorPage {
        const NAME: &'static str = "OriGalleryLoadingIndicatorPage";
        type Type = super::LoadingIndicatorPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LoadingIndicatorPage {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for LoadingIndicatorPage {}
    impl BinImpl for LoadingIndicatorPage {}
}

glib::wrapper! {
    pub struct LoadingIndicatorPage(ObjectSubclass<imp::LoadingIndicatorPage>)
        @extends adw::Bin, gtk::Widget;
}
