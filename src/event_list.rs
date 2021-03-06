use crate::event::Event;
type Link<T> = Option<Box<ListElement<T>>>;
type LinkRef<'a, T> = Option<&'a Box<ListElement<T>>>;
type LinkRefMut<'a, T> = Option<&'a mut Box<ListElement<T>>>;
#[derive(Debug, Clone)]
pub struct List<T> {
    pub head: Link<T>,
}
pub type EventList = List<Event>;
impl<T> List<T> {
    /// Creates new list with head
    pub fn new_with_head(head: ListElement<T>) -> Self {
        Self {
            head: Some(Box::new(head)),
        }
    }
}
impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let mut vec: Vec<Self::Item> = Vec::new();
        let mut current_link = self.head;
        while let Some(current) = current_link {
            vec.push(current.value);
            current_link = current.next;
        }
        vec.into_iter()
    }
}
#[derive(Debug, Clone)]
pub struct ListElement<T> {
    pub value: T,
    next: Link<T>,
}
impl<T> ListElement<T> {
    /// Creates new list element
    /// * 'value' - value to be stored inside
    pub fn new(value: T) -> Self {
        Self {
            value,
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
    pub fn next_ref(&self) -> LinkRef<T> {
        self.next.as_ref()
    }
}
