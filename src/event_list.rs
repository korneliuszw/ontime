use crate::event::Event;
use std::cell::RefCell;
use std::rc::Rc;
type Link<T> = Option<Box<ListElement<T>>>;
type LinkRef<'a, T> = Option<&'a Box<ListElement<T>>>;
type LinkRefMut<'a, T> = Option<&'a mut Box<ListElement<T>>>;
type ValueType<T> = Rc<RefCell<T>>;
#[derive(Debug, Clone)]
pub struct List<T> {
    pub head: Link<T>,
}
#[derive(Debug, Clone)]
pub struct ListElement<T> {
    pub value: ValueType<T>,
    next: Link<T>,
}
pub type EventList = List<Event>;
impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }
    /// Creates new list with head
    pub fn new_with_head(head: ListElement<T>) -> Self {
        Self {
            head: Some(Box::new(head)),
        }
    }
    pub fn pop(&mut self) -> Option<ValueType<T>> {
        match self.head.take() {
            None => None,
            Some(head) => {
                self.head = head.next;
                Some(head.value)
            }
        }
    }
}
impl<T> ListElement<T> {
    /// Creates new list element
    /// * 'value' - value to be stored inside
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
            next: None,
        }
    }
    /// Adds element to the next field
    /// * 'element' - element to add
    /// Returns mutable reference to newly added element
    pub fn push(&mut self, element: Self) -> LinkRefMut<T> {
        self.next = Some(Box::new(element));
        return self.next_mut();
    }
    pub fn next_mut(&mut self) -> LinkRefMut<T> {
        self.next.as_mut()
    }
    pub fn next(&self) -> LinkRef<T> {
        self.next.as_ref()
    }
    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }
    pub fn pop(&mut self) -> Option<()> {
        match self.next.take() {
            None => None,
            Some(x) => {
                self.value = x.value;
                self.next = x.next;
                Some(())
            }
        }
    }
}