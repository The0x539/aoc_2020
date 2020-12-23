use crate::prelude::*;

use std::{cell::RefCell, rc::Rc};

type R<T> = Rc<RefCell<T>>;

pub struct Node<T> {
    val: T,
    next: Option<R<Self>>,
    referent: Option<R<Self>>,
}

impl<T> Node<T> {
    fn get_next(&self) -> Option<R<Self>> {
        self.next.as_ref().map(Rc::clone)
    }

    fn get_referent(&self) -> Option<R<Self>> {
        self.referent.as_ref().map(Rc::clone)
    }

    fn set_next(&mut self, next: &R<Self>) {
        self.next = Some(Rc::clone(next));
    }

    fn take_next(&mut self) -> Option<R<Self>> {
        let next = self.get_next()?;
        self.next = next.borrow().get_next();
        Some(next)
    }

    fn insert_after(&mut self, n: R<Self>) {
        n.borrow_mut().next = self.get_next();
        self.next = Some(n);
    }

    fn from_iter(iter: impl IntoIterator<Item = T>) -> R<Self> {
        let mut iter = iter.into_iter();
        let head = Rc::new(RefCell::new(Self {
            val: iter.next().expect("pls"),
            next: None,
            referent: None,
        }));
        head.borrow_mut().set_next(&head);

        let mut tail = Rc::clone(&head);
        for val in iter {
            let n = Rc::new(RefCell::new(Self {
                val,
                next: Some(Rc::clone(&head)),
                referent: None,
            }));
            tail.borrow_mut().set_next(&n);
            tail = n;
        }

        head
    }
}

macro_rules! next {
    ($n:ident) => {{
        let next__ = $n.borrow().get_next().unwrap();
        $n = next__;
    }};
}

impl Node<u32> {
    fn establish_referent(&mut self, max: u32) {
        let mut n = self.get_next().unwrap();
        loop {
            if n.borrow().val == self.val - 1 || (self.val == 1 && n.borrow().val == max) {
                self.referent = Some(Rc::clone(&n));
                return;
            }
            next!(n);
        }
    }
}

fn establish_referents(head: &R<Node<u32>>, max: u32) {
    head.borrow_mut().establish_referent(max);
    let mut n = head.borrow().get_next().unwrap();
    while !Rc::ptr_eq(&n, head) {
        n.borrow_mut().establish_referent(max);
        next!(n);
    }
}

fn do_move(n: &mut Node<u32>) {
    let picked = [
        n.take_next().unwrap(),
        n.take_next().unwrap(),
        n.take_next().unwrap(),
    ];

    let mut referent = n.get_referent().unwrap();
    while picked.iter().any(|x| Rc::ptr_eq(x, &referent)) {
        let refref = referent.borrow().get_referent().unwrap();
        referent = refref;
    }

    let [a, b, c] = picked;
    referent.borrow_mut().insert_after(c);
    referent.borrow_mut().insert_after(b);
    referent.borrow_mut().insert_after(a);
}

pub enum Day23 {}

impl Challenge for Day23 {
    type Input = Vec<u32>;
    type Output1 = Vec<u32>;
    type Output2 = u64;

    fn read(data: File) -> Result<Self::Input, Error> {
        Ok(data
            .bytes()
            .map(Result::unwrap)
            .filter(u8::is_ascii_digit)
            .map(|n| (n - b'0') as u32)
            .collect())
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        let mut n = Node::from_iter(input);
        establish_referents(&n, 9);

        for _ in 0..100 {
            do_move(&mut n.borrow_mut());
            next!(n);
        }

        while n.borrow().val != 1 {
            next!(n);
        }
        next!(n);

        let mut v = Vec::new();
        while n.borrow().val != 1 {
            v.push(n.borrow().val);
            next!(n);
        }
        v
    }

    fn part2(mut input: Self::Input) -> Self::Output2 {
        // Establish the initial 9 nodes, plus a guaranteed 10 at the end.
        input.push(10);
        let mut n = Node::from_iter(input);
        establish_referents(&n, 10);

        // Get a reference to the currently final node. Whatever.
        let mut tail = Rc::clone(&n);
        for _ in 0..9 {
            next!(tail); // advance to the actual tail
        }
        assert_eq!(tail.borrow().val, 10);

        // Add the remaining 999990 nodes.
        // Doing it like this prevents O(n) time complexity for establishing each referent,
        // since with a singly linked list you'd have to wrap around and all that.
        // If we know it to be sorted, then whatever.
        for i in 11..=1_000_000 {
            let new_node = Rc::new(RefCell::new(Node {
                val: i,
                next: Some(Rc::clone(&n)),
                referent: Some(Rc::clone(&tail)),
            }));
            tail.borrow_mut().set_next(&new_node);
            tail = new_node;
        }

        // Set up the n=1 node to point back at the new final node.
        let mut one = Rc::clone(&n);
        while one.borrow().val != 1 {
            next!(one);
        }
        one.borrow_mut().referent = Some(tail);

        // Do the thing!
        for _ in 0..10_000_000 {
            do_move(&mut n.borrow_mut());
            next!(n);
        }

        while n.borrow().val != 1 {
            next!(n);
        }
        next!(n);
        let a = n.borrow().val;
        next!(n);
        let b = n.borrow().val;
        a as u64 * b as u64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day23 as Challenge>::Input {
        vec![3, 8, 9, 1, 2, 5, 4, 6, 7]
    }

    #[test]
    fn test_day23_part1() {
        assert_eq!(Day23::part1(sample_input()), vec![6, 7, 3, 8, 4, 5, 2, 9]);
    }

    #[test]
    fn test_day23_part2() {
        assert_eq!(Day23::part2(sample_input()), 149245887792);
    }
}
