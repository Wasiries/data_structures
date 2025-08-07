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
    use std::{rc::Rc, cell::RefCell};
    struct Single<T: Clone> {
        data: T,
        next: Option<Rc<RefCell<Single<T>>>>,
    }
    pub struct SinglyLinkedList<T: Clone> {
        root: Option<Rc<RefCell<Single<T>>>>,
        amount: usize,
    }
    impl<T> SinglyLinkedList<T> where T: Clone {
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
        pub fn push_to(&mut self, data: T, index: usize) {
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
        

    }
}