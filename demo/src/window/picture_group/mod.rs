use adw::subclass::prelude::*;
use glib::clone;
use gtk::{gdk, glib};

mod imp {

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/window/picture_group/picture_group.blp")]
    pub struct PictureGroupPage {
        #[template_child]
        pub(super) group: TemplateChild<ori::Group>,
        #[template_child]
        pub(super) drop_target: TemplateChild<gtk::DropTarget>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PictureGroupPage {
        const NAME: &'static str = "OriDemoPictureGroupPage";
        type Type = super::PictureGroupPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PictureGroupPage {
        fn constructed(&self) {
            self.parent_constructed();

            self.drop_target.connect_drop(
                clone!(@to-owned self as imp => @default-return false, move
                    |_, value, _, _ | {
                        let Ok(file_list) = value.get::<gdk::FileList>() else { return false; };

                        let files = file_list.files();

                        let pictures = files.iter().map(|file| {
                            gtk::Picture::builder()
                            .file(file)
                            .content_fit(gtk::ContentFit::Cover)
                            .overflow(gtk::Overflow::Hidden)
                            .css_classes(["card"])
                            .build()
                        });

                        imp.group.replace_children(pictures);

                        true
                    }
                ),
            );
        }
    }
    impl WidgetImpl for PictureGroupPage {}
    impl BinImpl for PictureGroupPage {}
}

glib::wrapper! {
    pub struct PictureGroupPage(ObjectSubclass<imp::PictureGroupPage>)
        @extends gtk::Widget;
}
