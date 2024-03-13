struct Node<T> {
    value: T, //pode ser string, ou qualquer outra coisa
    next: Option<Box<Node<T>>>, // pode ou não existir o next para outro node
}

struct Stack<T> {
    top: Option<Box<Node<T>>>,
}

fn main() {
    let str = String::from("Nicolas");

    if is_palindrome(&str) {
        println!("A palavra {str} é palindroma!");
    } else {
        println!("A palavra {str} não é palindroma!")
    }
    
}

fn is_palindrome(str: &String) -> bool { // n perco a propriedade do struct na main
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
        Node { value: value, next:None } // ou posso simplificar só com um value
    }
}

impl<T> Stack<T> {
    fn new() -> Self { //retorna uma instancia da propria estrutura
        Stack { top: None }
    }

    fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    // inserir dados na pilha, deve ser mut pq vamos mudar ela
    fn push(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));

        if let Some(top) = self.top.take() {
            new_node.next = Some(top);
        }

        self.top = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None
        }

        let removed_node = self.top.take(); //pegamos o valor do top

        match removed_node {
            Some(node) => {
                self.top = node.next;
                Some(node.value) // retornamos o valor do nó removido
            }
            None => None,
        }
    }
}

