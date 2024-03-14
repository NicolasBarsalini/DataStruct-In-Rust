struct Node<T> {
    value: T, // The value can be string or anything else.
    next: Option<Box<Node<T>>>, // This value points to another Node value. Therefore, it may or may not exist 'next' to the next node
}

struct Stack<T> {
    top: Option<Box<Node<T>>>,
}

fn main() {
    let str = String::from("Edgar");

    if is_palindrome(&str) {
        println!("the word {str} is a palindrome!");
    } else {
        println!("The word {str} is not a palindrome!")
    }
    
}

fn is_palindrome(str: &String) -> bool { // Do not lose the property of struct in main! You LEND the value, and the function BORROW the value.
    let mut stack = Stack::new();

    for letter in str.chars() {
        stack.push(letter);
    }

    let mut str_reversed = String::new();

    while !stack.is_empty() {
        let letter = match stack.pop() {
            Some(v) => v,
            None => continue,
        };

        str_reversed.push(letter);
    }

    println!("{str_reversed}");

    str.eq(&str_reversed)
}

impl<T> Node<T> {
    // "Constructor"
    fn new(value:T)-> Self {
        Node { value: value, next:None } // I also can simplify by just putting 'value'
    }
}

impl<T> Stack<T> {
    fn new() -> Self { // returns a instance of the struct itself
        Stack { top: None } 
    }

    fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    // insert data into the Stack, it must be MUT because we will change it.
    fn push(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));

        if let Some(top) = self.top.take() {
            new_node.next = Some(top);
        }

        self.top = Some(new_node);
    }

    // remove the last element of the Stack
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None
        }

        let removed_node: Option<Box<Node<T>>> = self.top.take(); //pegamos o valor do top

        match removed_node {
            Some(node) => {
                self.top = node.next;
                Some(node.value) // retornamos o valor do nó removido
            }
            None => None,
        }
    }
}

