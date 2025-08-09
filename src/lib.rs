pub mod stack {
    use std::{cell::RefCell, rc::Rc};
    struct Node<T: Clone> {
        data: T,
        next: Option<Rc<RefCell<Node<T>>>>,
    }
    pub struct Stack<T: Clone> {
        root: Option<Rc<RefCell<Node<T>>>>,
        amount: usize,
    }
    impl<T> Stack<T> where T: Clone {
        pub fn new() -> Self {
            return Stack {
                root: None,
                amount: 0,
            };
        }
        pub fn push(&mut self, data: T) {
            match &self.root {
                None => {
                    self.root = Some(Rc::new(RefCell::new(Node {
                        data,
                        next: None,
                    })));
                },
                Some(value) => {
                    let node = Rc::new(RefCell::new(Node {
                        data,
                        next: Some(Rc::clone(value)),
                    }));
                    self.root = Some(Rc::clone(&node));
                }
            }
            self.amount += 1;
        }
        pub fn get(&mut self) -> Result<T, Box<dyn std::error::Error>> {
            match self.root.take() {
                None => {
                    return Err(Box::new(std::fmt::Error));
                },
                Some(value) => {
                    let data = value.borrow().data.clone();
                    let next = &value.borrow().next;
                    match next {
                        None => {
                            self.root = None;
                        },
                        Some(content) => {
                            self.root = Some(Rc::clone(content));
                        }
                    }
                    self.amount -= 1;
                    return Ok(data);
                }
            }
        }
        pub fn top(&self) -> Result<T, Box<dyn std::error::Error>> {
            match &self.root {
                None => {
                    return Err(Box::new(std::fmt::Error));
                },
                Some(value) => {
                    return Ok(value.borrow().data.clone());
                }
            }
        }
        pub fn size(&self) -> &usize {
            return &self.amount;
        }

    }
}
pub mod queue {
    use std::{cell::RefCell, rc::Rc};
    struct Node<T: Clone> {
        data: T,
        next: Option<Rc<RefCell<Node<T>>>>,
    }
    pub struct Queue<T: Clone> {
        root: Option<Rc<RefCell<Node<T>>>>,
        amount: usize,
    }
    impl<T> Queue<T> where T: Clone {
        pub fn new() -> Self {
            return Queue {
                root: None,
                amount: 0,
            };
        }
        pub fn push(&mut self, data: T) {
            match &self.root {
                None => {
                    self.root = Some(Rc::new(RefCell::new(Node {
                        data,
                        next: None,
                    })));
                },
                Some(temp) => {
                    let mut current = Some(Rc::clone(temp));
                    while let Some(value) = current {
                        match &value.borrow().next {
                            None => {
                                value.borrow_mut().next = Some(Rc::new(RefCell::new(Node {
                                    data,
                                    next: None,
                                })));
                                return;
                            },
                            Some(next) => {
                                current = Some(Rc::clone(next));
                            }
                        }
                    }
                }
            }
        }
        pub fn get(&mut self) -> Result<T, Box<dyn std::error::Error>> {
            match self.root.take() {
                None => {
                    return Err(Box::new(std::fmt::Error));
                },
                Some(value) => {
                    let temp = value.borrow().data.clone();
                    match &value.borrow().next {
                        None => {
                            self.root = None;
                        },
                        Some(content) => {
                            self.root = Some(Rc::clone(content));
                        }
                    }
                    self.amount -= 1;
                    return Ok(temp);
                }
            }
        }
        pub fn top(&self) -> Result<T, Box<dyn std::error::Error>> {
            match &self.root {
                None => {
                    return Err(Box::new(std::fmt::Error));
                },
                Some(value) => {
                    let temp = value.borrow().data.clone();
                    return Ok(temp);
                }
            }
        }
        pub fn size(&self) -> &usize {
            return &self.amount;
        }
    }
}
pub mod linked_list {
    use std::{rc::{Rc, Weak}, cell::RefCell};
    struct Single<T: Clone + std::fmt::Display + std::fmt::Debug> {
        data: T,
        next: Option<Rc<RefCell<Single<T>>>>,
    }
    pub struct SinglyLinkedList<T: Clone + std::fmt::Display + std::fmt::Debug> {
        root: Option<Rc<RefCell<Single<T>>>>,
        amount: usize,
    }
    impl<T> SinglyLinkedList<T> where T: Clone + std::fmt::Display + std::fmt::Debug {
        pub fn new() -> Self {
            return SinglyLinkedList {
                root: None,
                amount: 0,
            };
        }
        pub fn push_begin(&mut self, data: T) {
            match self.root.take() {
                None => {
                    self.root = Some(Rc::new(RefCell::new(Single {
                        data,
                        next: None,
                    })));
                },
                Some(value) => {
                    let node = Rc::new(RefCell::new(Single {
                        data,
                        next: Some(Rc::clone(&value)),
                    }));
                    self.root = Some(node);
                }
            }
            self.amount += 1;
        }
        pub fn push_back(&mut self, data: T) {
            match &self.root {
                None => {
                    self.root = Some(Rc::new(RefCell::new(Single {
                        data,
                        next: None,
                    })));
                },
                Some(value) => {
                    let mut current = Some(Rc::clone(value));
                    while let Some(temp) = current {
                        match &temp.borrow().next {
                            None => {
                                temp.borrow_mut().next = Some(Rc::new(RefCell::new(Single {
                                    data,
                                    next: None,
                                })));
                                return;
                            },
                            Some(next) => {
                                current = Some(Rc::clone(next));
                            }
                        }
                    }
                }
            }
            self.amount += 1;
        }
        pub fn insert(&mut self, data: T, index: usize) {
            if index >= self.amount {
                self.push_back(data);
            } else {
                if index == 0 {
                    self.push_begin(data);
                } else {
                    match &self.root {
                        None => {
                            self.root = Some(Rc::new(RefCell::new(Single {
                                data,
                                next: None,
                            })));
                        },
                        Some(value) => {
                            let mut current = Some(Rc::clone(value));
                            let mut counter = 1;
                            while let Some(temp) = current {
                                match &temp.borrow().next {
                                    None => {
                                        temp.borrow_mut().next = Some(Rc::new(RefCell::new(Single {
                                            data,
                                            next: None,
                                        })));
                                        return;
                                    },
                                    Some(content) => {
                                        if counter == index {
                                            let node = Rc::new(RefCell::new(Single {
                                                data,
                                                next: Some(Rc::clone(content)),
                                            }));
                                            temp.borrow_mut().next = Some(Rc::clone(&node));
                                            return;
                                        } else {
                                            current = Some(Rc::clone(content));
                                            counter += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        pub fn pop_begin(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            match self.root.take() {
                None => {
                    return Err(Box::new(std::fmt::Error));
                },
                Some(value) => {
                    match &value.borrow().next {
                        None => {
                            self.root = None;
                        },
                        Some(content) => {
                            self.root = Some(Rc::clone(content));
                        }
                    }
                    self.amount -= 1;
                    return Ok(());
                }
            }
        }
        pub fn pop_back(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            match self.root.clone() {
                None => {
                    return Err(Box::new(std::fmt::Error));
                },
                Some(value) => {
                    match &value.borrow().next {
                        None => {
                            self.root = None;
                        },
                        Some(_) => {
                            let mut current = Some(Rc::clone(&value));
                            while let Some(content) = current {
                                match &content.borrow().next {
                                    None => {
                                        return Err(Box::new(std::fmt::Error));
                                    },
                                    Some(temp) => {
                                        match &temp.borrow().next {
                                            None => {
                                                content.borrow_mut().next = None;
                                                break;
                                            },
                                            Some(_) => {
                                                current = Some(Rc::clone(temp));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self.amount -= 1;
                    return Ok(());
                }
            }
        }
        pub fn erase(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
            if index >= self.amount {
                return Err(Box::new(std::fmt::Error));
            }
            if index == 0 {
                return self.pop_begin();
            }
            match self.root.clone() {
                None => {
                    return Err(Box::new(std::fmt::Error));
                },
                Some(value) => {
                    match &value.borrow().next {
                        None => {
                            return Err(Box::new(std::fmt::Error));
                        },
                        Some(_) => {
                            let mut current = Some(Rc::clone(&value));
                            let mut counter = 1;
                            while let Some(temp) = current {
                                match &temp.borrow().next {
                                    None => {
                                        return Err(Box::new(std::fmt::Error));
                                    },
                                    Some(next) => {
                                        match &next.borrow().next {
                                            None => {
                                                temp.borrow_mut().next = None;
                                                break;
                                            },
                                            Some(further) => {
                                                if counter == index {
                                                    temp.borrow_mut().next = Some(Rc::clone(further));
                                                    break;
                                                } else {
                                                    current = Some(Rc::clone(next));
                                                    counter += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self.amount -= 1;
            return Ok(());
        }
        pub fn get(&self, index: usize) -> Result<T, Box<dyn std::error::Error>> {
            if index >= self.amount {
                return Err(Box::new(std::fmt::Error));
            }
            match &self.root {
                None => {
                    return Err(Box::new(std::fmt::Error));
                },
                Some(value) => {
                    let mut current = Some(Rc::clone(value));
                    let mut counter = 0;
                    while let Some(content) = current {
                        if counter == index {
                            return Ok(content.borrow().data.clone());
                        } else {
                            match &content.borrow().next {
                                None => {
                                    return Err(Box::new(std::fmt::Error));
                                },
                                Some(temp) => {
                                    current = Some(Rc::clone(temp));
                                    counter += 1;
                                }
                            }
                        }
                    }
                    return Ok(value.borrow().data.clone());
                }
            }
        }
        pub fn show(&self) {
            println!("=====================list=begin=====================");
            match &self.root {
                None => {
                    println!(" ");
                }, 
                Some(value) => {
                    let mut current = Some(Rc::clone(value));
                    while let Some(content) = current {
                        println!("{}", &content.borrow().data);
                        match &content.borrow().next {
                            None => {
                                break;
                            },
                            Some(next) => {
                                current = Some(Rc::clone(next));
                            }
                        }
                    }
                }
            }
            println!("======================list=end======================");
        }
        pub fn size(&self) -> &usize {
            return &self.amount;
        }
    }
    struct Double<T: Clone + std::fmt::Debug + std::fmt::Display> {
        data: T,
        prev: Option<Weak<RefCell<Double<T>>>>,
        next: Option<Rc<RefCell<Double<T>>>>,
    }
    pub struct DoublyLinkedList<T: Clone + std::fmt::Debug + std::fmt::Display> {
        head: Option<Rc<RefCell<Double<T>>>>,
        tail: Option<Weak<RefCell<Double<T>>>>,
        amount: usize,
    }
    impl<T> DoublyLinkedList<T> where T: Clone + std::fmt::Debug + std::fmt::Display {
        pub fn new() -> Self {
            return DoublyLinkedList {
                head: None,
                tail: None,
                amount: 0
            };
        }
        pub fn push_begin(&mut self, data: T) {
            match &self.head {
                None => {
                    let node = Rc::new(RefCell::new(Double{
                        data,
                        prev: None,
                        next: None,
                    }));
                    self.head = Some(Rc::clone(&node));
                    self.tail = Some(Rc::downgrade(&node));
                },
                Some(value) => {
                    let node = Rc::new(RefCell::new(Double {
                        data,
                        prev: None,
                        next: Some(Rc::clone(value)),
                    }));
                    value.borrow_mut().prev = Some(Rc::downgrade(&node));
                    self.head = Some(Rc::clone(&node));
                }
            }
            self.amount += 1;
        }
        pub fn push_back(&mut self, data: T) {
            match &self.tail {
                None => {
                    let node = Rc::new(RefCell::new(Double {
                        data,
                        prev: None,
                        next: None,
                    }));
                    self.head = Some(Rc::clone(&node));
                    self.tail = Some(Rc::downgrade(&node));
                },
                Some(value) => {
                    let node = Rc::new(RefCell::new(Double {
                        data,
                        prev: Some(Weak::clone(value)),
                        next: None,
                    }));
                    value.upgrade().unwrap().borrow_mut().next = Some(Rc::clone(&node));
                    self.tail = Some(Rc::downgrade(&node));
                }
            }
            self.amount += 1;
        }
        pub fn insert(&mut self, data: T, index: usize) {
            if index >= self.amount {
                self.push_back(data);
                return;
            }
            if index == 0 {
                self.push_begin(data);
                return;
            }
            match &self.head {
                None => {
                    let node = Rc::new(RefCell::new(Double {
                        data,
                        prev: None,
                        next: None,
                    }));
                    self.head = Some(Rc::clone(&node));
                    self.tail = Some(Rc::downgrade(&node));
                },
                Some(value) => {
                    let mut current = Some(Rc::clone(value));
                    let mut counter = 1;
                    while let Some(temp) = current {
                        match &temp.borrow().next {
                            None => {
                                let node = Rc::new(RefCell::new(Double {
                                    data,
                                    prev: Some(Rc::downgrade(&temp)),
                                    next: None,
                                }));
                                temp.borrow_mut().next = Some(Rc::clone(&node));
                                self.tail = Some(Rc::downgrade(&node));
                                break;
                            },
                            Some(content) => {
                                if counter == index {
                                    let node = Rc::new(RefCell::new(Double {
                                        data,
                                        prev: Some(Rc::downgrade(&temp)),
                                        next: Some(Rc::clone(content)),
                                    }));
                                    temp.borrow_mut().next = Some(Rc::clone(&node));
                                    content.borrow_mut().prev = Some(Rc::downgrade(&node));
                                    break;
                                }
                                current = Some(Rc::clone(content));
                                counter += 1;
                            }
                        }
                    }
                }
            }
            self.amount += 1;
        }
    }
}