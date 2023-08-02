use gtk::prelude::*;

pub struct ChildIter {
    current: Option<gtk::Widget>,
}

impl Iterator for ChildIter {
    type Item = gtk::Widget;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(result) = std::mem::take(&mut self.current) {
            self.current = result.next_sibling();
            Some(result)
        } else {
            None
        }
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
