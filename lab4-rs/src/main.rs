use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
};

type Link = Option<Rc<RefCell<Node>>>;

struct BinaryTree {
    root: Link,
    check_for_duble: Vec<i32>,
}

struct Node {
    data: i32,
    left: Link,
    right: Link,
}

impl BinaryTree {
    fn new() -> Self {
        BinaryTree {
            root: None,
            check_for_duble: Vec::new(),
        }
    }

    fn add(&mut self, data: i32) {
        if self.check_for_duble.contains(&data) {
            return;
        }
        self.check_for_duble.push(data);
        self.root = Self::add_node(self.root.take(), data);
    }

    fn add_node(node: Link, data: i32) -> Link {
        match node {
            Some(n) => {
                if data < n.borrow().data {
                    let left_child = n.borrow_mut().left.take();
                    n.borrow_mut().left = Self::add_node(left_child, data);
                } else {
                    let right_child = n.borrow_mut().right.take();
                    n.borrow_mut().right = Self::add_node(right_child, data);
                }
                Some(n)
            }
            None => Some(Rc::new(RefCell::new(Node {
                data,
                left: None,
                right: None,
            }))),
        }
    }

    fn print(&self) {
        if self.root.is_none() {
            return;
        }
        Self::print_node(&self.root, 0);
    }

    fn print_node(node: &Link, l: usize) {
        if let Some(n) = node {
            Self::print_node(&n.borrow().right, l + 1);
            for _ in 0..l {
                print!(" ");
            }
            println!("{}", n.borrow().data);
            Self::print_node(&n.borrow().left, l + 1);
        }
    }

    fn search(&mut self, data: i32) -> i32 {
        let mut count = 0;
        self.root = Self::search_and_remove(self.root.take(), data, &mut count);
        count
    }

    fn search_and_remove(node: Link, data: i32, count: &mut i32) -> Link {
        match node {
            Some(n) => {
                if n.borrow().data == data && *count == 0 {
                    *count += 1;
                    return None; // Удаляем узел, если найден дубликат
                }
                let left_child = n.borrow_mut().left.take();
                n.borrow_mut().left = Self::search_and_remove(left_child, data, count);
                if *count == 0 {
                    let right_child = n.borrow_mut().right.take();
                    n.borrow_mut().right = Self::search_and_remove(right_child, data, count);
                }
                Some(n)
            }
            None => None,
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    println!("Начало генерации дерева (-1 для прерывания)");

    loop {
        print!("Введите число > ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input == -1 {
            println!("Генерация прервана\n");
            break;
        } else {
            tree.add(input);
        }
    }

    tree.print();

    println!("\n{}", tree.search(12));
}
