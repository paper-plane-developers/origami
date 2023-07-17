use adw::subclass::prelude::*;
use gtk::glib;

macro_rules! page {
    ($name:ident, $template:literal, $imp:ident) => {
        mod $imp {
            use super::*;

            #[derive(Debug, Default, gtk::CompositeTemplate)]
            #[template(file = $template)]
            pub struct $name;

            #[glib::object_subclass]
            impl ObjectSubclass for $name {
                const NAME: &'static str = concat!("OriDemo", stringify!($name));
                type Type = super::$name;
                type ParentType = adw::Bin;

                fn class_init(klass: &mut Self::Class) {
                    klass.bind_template();
                }

                fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
                    obj.init_template();
                }
            }

            impl ObjectImpl for $name {}
            impl WidgetImpl for $name {}
            impl BinImpl for $name {}
        }

        glib::wrapper! {
            pub struct $name(ObjectSubclass<$imp::$name>)
                @extends adw::Bin, gtk::Widget;
        }
    };
}

page!(
    ShimmerEffectPage,
    "src/window/simple_pages/shimmer_effect.blp",
    imp1
);

page!(
    LoadingIndicatorPage,
    "src/window/simple_pages/loading_indicator.blp",
    imp2
);
