pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Node {
            value,
            next
        }
    }
}

impl<T> SimpleLinkedList<T> {

    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            len: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Box::new(Node::new(_element, self.head.take()));
        self.len += 1;
        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|box_node| {
            let Node { value, next  } = *box_node;

            self.head = next;
            self.len -= 1;

            value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();
        let mut current = self.head;

        while let Some(mut box_node) = current {
            current = box_node.next.take();
            rev_list.push(box_node.value)
        }

        rev_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        _iter.into_iter().for_each(|x| list.push(x));

        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut result = vec![];

        while let Some(node) = _linked_list.pop() {
            result.push(node);
        }

        result.reverse();

        result
    }
}
