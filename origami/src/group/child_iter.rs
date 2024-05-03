use gtk::prelude::*;

pub struct ChildIter {
    current: Option<gtk::Widget>,
}

impl Iterator for ChildIter {
    type Item = gtk::Widget;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|w| {
            self.current = w.next_sibling();
            w
        })
    }
}

pub trait WidgetIterExt: WidgetExt {
    fn iter_children(&self) -> ChildIter {
        ChildIter {
            current: self.first_child(),
        }
    }
}

impl<T> WidgetIterExt for T where T: WidgetExt {}
