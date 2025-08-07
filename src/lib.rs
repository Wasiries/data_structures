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