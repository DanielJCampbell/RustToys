//Linked-List Stack code taken from http://cglab.ca/~abeinges/blah/too-many-lists/book/second-final.html
//Just used as a template I can stick macros onto

pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(Stack<T>);

pub struct Iter<'a, T:'a> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}



impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl <T: PartialEq> Stack<T> {

    pub fn contains(&self, elem: T) -> bool {
        let mut cur = self.head.as_ref();
        while let Some(boxed) = cur {
            if boxed.elem == elem {
                return true
            }
            cur = boxed.next.as_ref();
        }
        false
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

//Mimics the behaviour of vec!, for a stack
macro_rules! stack {
    //No idea why this wants type information.
    //Perhaps a difference in implementation between Stack::new and Vec::new?
    ($t: ty) => {{
        Stack::<$t>::new()
    }};

    ($t: ty; $e:expr; $n:expr) => {{
        let mut tmp = Stack::<$t>::new();
        for _ in 0..$n {
            tmp.push($e.clone());
        }
        tmp
    }};

    ($t: ty, $($e: expr),*) => {{
        let mut tmp : Stack<$t> = Stack::new();
        $(
            tmp.push($e);
        )*
        tmp
    }};

}

//Given an identifier pointing to a stack and a non-empty sequence of
//expressions, add all the expressions to the stack (if typing permits)
macro_rules! stack_add_all {

    ($s:ident, $($e: expr),+) => {{
        $($s.push($e);)*
    }};
}

//Given an identifier pointing to a stack and a non-empty sequence of
//expressions, return true if any of the expressions is in the stack, otherwise false
macro_rules! stack_contains {
    ($s:ident, $($e: expr),+) => {{
        let mut result = false;
        $(
            for &elem in $s.iter() {
                if $e == elem {
                    result = true;
                }
            }
        )*
        result
    }};
}

//As above, but returns true iff all elements are in the stack
macro_rules! stack_contains_all {
    ($s:ident, $($e: expr),+) => {{
        let mut result = true;
        $(
            if !stack_contains!($s, $e) {
                result = false;
            }
        )*
        result
    }};
}

//Designed to emulate fizz-buzz, or any variant thereof (Returns a string)
//First expr must be some kind of integer iterator (eg. a range)
//Then a sequence of tuples of the form (number, string).
//For every number in the range, will append a line: either all strings
//for the numbers it is a multiple of, or just the number itself
macro_rules! fizz_buzz {
    ($e:expr, $(($n:expr, $s:expr)),*) => {{
        let mut output = "".to_owned();
        for i in $e {
            let mut tmp = "".to_owned();
            $(
                if i % $n == 0 {
                    tmp.push_str($s);
                } 
            )*
            if tmp.len() == 0 {
                tmp = i.to_string();
            }
            tmp.push_str("\n");
            output.push_str(&tmp);
        }
        output
    }};
}


#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn basics() {
        let mut stack = Stack::new();

        // Check empty list behaves right
        assert_eq!(stack.pop(), None);

        // Populate list
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Check normal removal
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        stack.push(4);
        stack.push(5);

        // Check normal removal
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));

        // Check exhaustion
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);
        stack.push(1); stack.push(2); stack.push(3);

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.peek_mut(), Some(&mut 3));
    }

    #[test]
    fn contains(){
        let mut stack = stack![i32, 1, 2, 3, 4, 5];
        assert!(stack.contains(1));
        assert!(!stack.contains(6));
        stack.pop();
        assert!(!stack.contains(5));
    }

    #[test]
    fn into_iter() {
        let mut stack = Stack::new();
        stack.push(1); stack.push(2); stack.push(3);

        let mut iter = stack.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn iter() {
        let mut stack = Stack::new();
        stack.push(1); stack.push(2); stack.push(3);

        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut stack = Stack::new();
        stack.push(1); stack.push(2); stack.push(3);

        let mut iter = stack.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

    #[test]
    fn macro_stack() {
        let mut stack = stack![i32, 1, 2, 3];
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));

        let stack = stack![i32];
        assert_eq!(stack.peek(), None);
        
        let mut stack = stack![i32; 0; 4];
        for _ in 0..4 {
            assert_eq!(stack.pop(), Some(0));
        }
    }

    #[test]
    fn macro_add_all() {
        let mut stack = Stack::new();
        stack_add_all!(stack, 1, 2, 3);
        assert_eq!(stack.pop(), Some(3));
        stack_add_all!(stack, 4, 5, 6);
        assert_eq!(stack.pop(), Some(6));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(2));
    }

    #[test]
    fn macro_contains() {
        let stack = stack![i32, 1, 2, 3];
        assert!(stack_contains!(stack, 1, 3));
        assert!(stack_contains!(stack, 2, 1, 3));
        assert!(stack_contains!(stack, 4, 5, 6, 3));
        assert!(!stack_contains!(stack, 4, 5));
        assert!(stack_contains_all!(stack, 1, 2, 3));
        assert!(!stack_contains_all!(stack, 1, 2, 3, 4));
    }

    #[test]
    fn macro_fizzbuzz() {
        let fizzbuzz = fizz_buzz!(1..16, (3, "Fizz"), (5, "Buzz"));
        assert_eq!(fizzbuzz, "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz\n");
    }
}