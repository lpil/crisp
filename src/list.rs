use std::rc::Rc;

//
// Adapted from Alexis Beingessner's "Learning Rust With
// Entirely Too Many Linked Lists".
//
// https://github.com/rust-unofficial/too-many-lists
//

/// A persistent singly linked list.
///
pub struct List<T> {
    head: Link<T>,
}

// Currently the link uses Rc, a reference counter.
// This could later be replaced with a garbage collector type.
// Real GC should have higher throughput than RC.
//
type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// Note: The deallocation of the linked list may not be
// tail recursive, if so a large list could blow the stack
// when it is dropped.
//
impl<T> List<T> {
    /// Construct a new empty list.
    ///
    pub fn new() -> Self {
        List { head: None }
    }

    /// Prepend an element to the begining of the list.
    ///
    pub fn cons(&self, elem: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    /// Fetch the first element of the list, if there is one.
    ///
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// Fetch the tail of the list.
    /// i.e. the list without the first element.
    ///
    pub fn tail(&self) -> Self {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    /// Indicate whether the list has any elements or not.
    ///
    pub fn is_empty(&self) -> bool {
        self.head().is_none()
    }
}

// NOTE: This implement's the Iter trait. May be useful later.
//
//  // Goes in the impl block
//  pub fn iter(&self) -> Iter<T> {
//    Iter { next: self.head.as_ref().map(|node| &**node) }
//  }
//
// pub struct Iter<'a, T: 'a> {
//     next: Option<&'a Node<T>>,
// }
//
// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.map(|node| {
//             self.next = node.next.as_ref().map(|node| &**node);
//             &node.elem
//         })
//     }
// }


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn cons_head_and_tail() {
        let list1 = List::new();
        assert_eq!(list1.head(), None);

        let list2 = list1.cons(1).cons(2).cons(3);
        assert_eq!(list2.head(), Some(&3));

        let list3 = list2.tail();
        assert_eq!(list3.head(), Some(&2));

        let list4 = list3.tail();
        assert_eq!(list4.head(), Some(&1));

        let list5 = list4.tail();
        assert_eq!(list5.head(), None);

        let list6 = list5.tail();
        assert_eq!(list6.head(), None);
    }

    #[test]
    fn lists_persist_after_cons() {
        let list1 = List::new();
        assert_eq!(list1.head(), None);

        let list2 = list1.cons(1).cons(2).cons(3);
        assert_eq!(list2.head(), Some(&3));
        assert_eq!(list1.head(), None);

        let list3 = list2.tail().tail();
        assert_eq!(list3.head(), Some(&1));
        assert_eq!(list2.head(), Some(&3));
        assert_eq!(list1.head(), None);
    }

    #[test]
    fn is_empty() {
        let list = List::new();
        assert_eq!(list.is_empty(), true);
        assert_eq!(list.cons(1).is_empty(), false);
    }
}
