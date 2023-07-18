use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod range_item_imp {
    use super::*;

    use std::cell::Cell;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::RangeItem)]
    pub struct RangeItem {
        #[property(get, set)]
        value: Cell<i32>,

        #[property(get = |imp: &Self| imp.value.get().to_string().into())]
        string: std::marker::PhantomData<glib::GString>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RangeItem {
        const NAME: &'static str = "OriRangeItem";
        type Type = super::RangeItem;
    }

    impl ObjectImpl for RangeItem {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            Self::derived_set_property(&self, id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            Self::derived_property(&self, id, pspec)
        }
    }
}

glib::wrapper! {
    pub struct RangeItem(ObjectSubclass<range_item_imp::RangeItem>);
}

impl RangeItem {
    pub fn new(value: i32) -> Self {
        glib::Object::builder().property("value", value).build()
    }
}

mod imp {
    use super::*;

    use std::cell::Cell;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::RangeModel)]
    pub struct RangeModel {
        #[property(get, set)]
        min: Cell<i32>,
        #[property(get, set)]
        max: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RangeModel {
        const NAME: &'static str = "OriRangeModel";
        type Type = super::RangeModel;

        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for RangeModel {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            Self::derived_set_property(&self, id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            Self::derived_property(&self, id, pspec)
        }
    }

    impl ListModelImpl for RangeModel {
        fn item_type(&self) -> glib::Type {
            RangeItem::static_type()
        }

        fn n_items(&self) -> u32 {
            (self.max.get() - self.min.get() + 1)
                .try_into()
                .expect("min should be less than max")
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            let value = self.min.get() + position as i32;

            (value <= self.max.get()).then_some(RangeItem::new(value).upcast())
        }
    }
}

glib::wrapper! {
    pub struct RangeModel(ObjectSubclass<imp::RangeModel>)
    @implements gio::ListModel;
}

impl RangeModel {
    pub fn new(min: i32, max: i32) -> Self {
        glib::Object::builder()
            .property("min", min)
            .property("max", max)
            .build()
    }
}
