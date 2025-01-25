use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk::gdk;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/window/spoiler/spoiler.blp")]
    pub struct SpoilerPage {
        #[template_child]
        pub(super) spoiler: TemplateChild<ori::SpoilerOverlay>,
        #[template_child]
        pub(super) drop_target: TemplateChild<gtk::DropTarget>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SpoilerPage {
        const NAME: &'static str = "OriDemoSpoilerPage";
        type Type = super::SpoilerPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SpoilerPage {
        fn constructed(&self) {
            self.parent_constructed();

            self.drop_target
                .connect_drop(clone!(@to-owned self as imp, #[upgrade_or] false, move,
                    |_, value, _, _ | {
                        let Ok(file_list) = value.get::<gdk::FileList>() else { return false; };

                        let files = file_list.files();

                        if let Some(picture) = imp.spoiler
                            .child()
                            .and_downcast::<gtk::Picture>() {
                                picture.set_file(files.first());
                                imp.spoiler.refresh_blur();
                        } else {
                            let picture = gtk::Picture::for_file(files.first().unwrap());
                            picture.set_content_fit(gtk::ContentFit::Cover);
                            imp.spoiler.set_child(Some(&picture));
                        }

                        if !imp.spoiler.hidden() {
                            imp.spoiler.set_hidden(true);
                        }

                        true
                    }
                ));
        }
    }

    impl WidgetImpl for SpoilerPage {}
    impl BinImpl for SpoilerPage {}

    #[gtk::template_callbacks]
    impl SpoilerPage {
        #[template_callback]
        fn hide_content(&self) {
            self.spoiler.set_hidden(true);
        }
    }
}

glib::wrapper! {
    pub struct SpoilerPage(ObjectSubclass<imp::SpoilerPage>)
        @extends adw::Bin, gtk::Widget;
}
