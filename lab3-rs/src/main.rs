use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    information: String,
    priority: i32,
    next: Link,
    previous: Link,
}

#[derive(Clone)]
struct Vec {
    head: Link,
    tail: Link,
    length: i32,
}

impl Vec {
    // новый пустой вектор
    fn new() -> Self {
        Vec {
            head: None,
            tail: None,
            length: 0,
        }
    }

    // дернуть сверху
    fn pop(&mut self) {
        if self.tail.is_none() {
            return;
        }

        let tail_node = self.tail.take().unwrap();
        let tail_node_borrowed = tail_node.borrow();

        if let Some(prev_node) = tail_node_borrowed.previous.clone() {
            prev_node.borrow_mut().next = None;
            self.tail = Some(prev_node);
        } else {
            self.head = None;
        }

        self.length -= 1;
    }

    // дернуть снизу
    fn rem(&mut self) {
        if self.head.is_none() {
            return;
        }

        let new_head = self
            .head
            .as_ref()
            .and_then(|node| node.borrow().next.clone());
        self.head = new_head.clone();

        if let Some(new_head_node) = new_head {
            new_head_node.borrow_mut().previous = None;
        } else {
            self.tail = None;
        }

        self.length -= 1;
    }

    // запушить наверх и по**р на приоритет
    fn push(&mut self, inf: String) {
        let new_node = Rc::new(RefCell::new(Node {
            information: inf,
            priority: -1,
            next: None,
            previous: self.tail.clone(),
        }));

        if let Some(old_tail) = self.tail.take() {
            old_tail.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().previous = Some(old_tail);
            self.tail = Some(new_node);
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }

        self.length += 1;
    }

    // получить значение хвоста
    fn top(&self) -> Option<(String, i32)> {
        self.head.as_ref()?;

        let tail_node = self.tail.as_ref().unwrap().borrow();
        Some((tail_node.information.clone(), tail_node.priority))
    }

    // просто потому что могу получить значение первого элемента
    fn bottom(&self) -> Option<(String, i32)> {
        self.head.as_ref()?;
        let head_node = self.head.as_ref().unwrap().borrow();
        Some((head_node.information.clone(), head_node.priority))
    }

    // добавить, учитывая приоритет чем больше - тем ближе к голове 0 - самый конец, pop (-1) всегда в конце
    fn add(&mut self, inf: String, priority: u32) {
        let new_node = Rc::new(RefCell::new(Node {
            information: inf,
            priority: priority as i32,
            next: None,
            previous: None,
        }));

        self.length += 1;

        if self.head.is_none() {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
            return;
        }

        let mut current = self.head.clone();
        let mut prev = None;

        while let Some(ref node) = current.clone() {
            if node.borrow().priority < priority as i32 {
                break;
            }
            prev = Some(current.clone());
            current.clone_from(&node.borrow().next);
        }

        if let Some(prev_node) = prev {
            new_node
                .borrow_mut()
                .next
                .clone_from(&prev_node.as_ref().unwrap().borrow().next);
            if let Some(_next_node) = new_node.borrow().next.clone() {
                new_node.borrow_mut().previous.clone_from(&prev_node);
            }
            new_node.borrow_mut().next.clone_from(&self.head);
            new_node.borrow_mut().previous.clone_from(&prev_node);
        } else {
            new_node.borrow_mut().next.clone_from(&self.head);
            if let Some(next_node) = new_node.borrow().next.clone() {
                next_node.borrow_mut().previous = Some(new_node.clone());
            }
            self.head = Some(new_node.clone());
        }

        if new_node.borrow().next.is_none() {
            self.tail = Some(new_node);
        }
    }

    fn len(&self) -> i32 {
        self.length
    }

    fn find(&self, search: String) {
        let mut current = self.head.clone();

        while let Some(ref node) = current.clone() {
            if node.borrow().information == search {
                println!("({} {})", node.borrow().information, node.borrow().priority);
            }
            current.clone_from(&node.borrow().next);
        }
    }
    fn invert(&mut self) {
        if self.head.is_none() {
            return;
        }

        let mut current = self.head.clone();
        while let Some(ref node) = current.clone() {
            let mut node_borrowed = node.borrow_mut();
            let next = node_borrowed.next.clone();
            node_borrowed.next = node_borrowed.previous.clone();
            node_borrowed.previous = next.clone();

            current = next.clone();
        }
        std::mem::swap(&mut self.head, &mut self.tail);
    }
}

fn main() {
    //пример создания нового "вектора""
    let mut local_vec = Vec::new();

    //пример добавления нового элемента, с учетом приоритера
    local_vec.add("privet".to_string(), 65);
    local_vec.add("mir".to_string(), 50);
    local_vec.add("privet mir".to_string(), 70);

    //пример push
    local_vec.push("inf".to_string());
    local_vec.push("12443".to_string());
    local_vec.push("fsfds".to_string());

    local_vec.find("inf".to_string());

    //вывод длины
    println!("{}", local_vec.len());

    //пример получения нижнего элемента
    if let Some(bottom) = local_vec.bottom() {
        println!("({}, {})", bottom.0, bottom.1);
    }

    //пример получения верхнего элемента
    if let Some(top) = local_vec.top() {
        println!("({}, {})", top.0, top.1);
    }

    //пример попы
    local_vec.pop();
    local_vec.rem();

    //вывод длины
    println!("{}", local_vec.len());

    local_vec.invert();

    //пример получения нижнего элемента
    if let Some(bottom) = local_vec.bottom() {
        println!("({}, {})", bottom.0, bottom.1);
    }

    //пример получения верхнего элемента
    if let Some(top) = local_vec.top() {
        println!("({}, {})", top.0, top.1);
    }
}
