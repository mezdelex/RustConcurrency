#[derive(Clone)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

pub struct Iter<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.take().map(|node| {
            let data = node.data;
            self.head = node.next;
            data
        })
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push_front(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = self.head.take();
        self.head = Some(new_node)
    }

    pub fn push_back(&mut self, data: T) {
        let mut current = &mut self.head;

        // pattern matching syntax
        // loop {
        //     match current {
        //         Some(node) => current = &mut node.next,
        //         None => break,
        //     }
        // }

        // guard pattern matching syntax (both do the same, but this is preferred for readability)
        while let Some(node) = current {
            current = &mut node.next;
        }

        *current = Some(Box::new(Node::new(data)));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let data = node.data;
            self.head = node.next;
            data
        })
    }

    pub fn iter(&self) -> Iter<T>
    where
        T: Clone,
    {
        Iter {
            head: self.head.clone(),
        }
    }
}
