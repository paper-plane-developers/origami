use gtk::glib;

use gtk::subclass::prelude::*;

mod imp {
    use std::cell::OnceCell;

    use super::*;

    #[derive(Default)]
    pub struct Formatter {
        pub(super) formatting_closure: OnceCell<Box<dyn Fn(i64) -> String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Formatter {
        const NAME: &'static str = "OriFormatter";
        type Type = super::Formatter;
    }

    impl ObjectImpl for Formatter {}
}

glib::wrapper! {
    pub struct Formatter(ObjectSubclass<imp::Formatter>);
}

impl Formatter {
    pub fn new<F>(fun: F) -> Self
    where
        F: Fn(i64) -> String + 'static,
    {
        let obj: Self = glib::Object::new();
        _ = obj.imp().formatting_closure.set(Box::new(fun));
        obj
    }

    pub fn format(&self, index: i64) -> String {
        if let Some(closure) = self.imp().formatting_closure.get() {
            (*closure)(index)
        } else {
            index.to_string()
        }
    }
}

impl Default for Formatter {
    fn default() -> Self {
        glib::Object::new()
    }
}
