#[derive(Debug, Clone)]
enum QNode<T>
where
    T: Clone + std::fmt::Debug,
{
    Tail { value: T },
    Body { value: T, next: *mut QNode<T> },
}

impl<T: Clone + std::fmt::Debug> QNode<T> {
    fn value(&self) -> T {
        match self {
            Self::Tail { value } | Self::Body { value, next: _ } => value.clone(),
        }
    }

    fn next(&self) -> Option<*mut Self> {
        match self {
            Self::Tail { value: _ } => None,

            Self::Body { value: _, next } => Some(*next),
        }
    }
}

#[derive(Debug, Clone)]
enum QueuePtr<T>
where
    T: Clone + std::fmt::Debug,
{
    Empty,
    Unit {
        value: T,
    },
    Multi {
        head_value: T,
        head_next: *mut QNode<T>,
        tail: *mut QNode<T>,
    },
}

#[derive(Debug, Clone)]
pub struct Queue<T>
where
    T: Clone + std::fmt::Debug,
{
    length: usize,
    ptr: QueuePtr<T>,
}

impl<T: Clone + std::fmt::Debug> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
    }
}

impl<T: Clone + std::fmt::Debug> Queue<T> {
    pub fn len(&self) -> usize {
        self.length
    }

    pub const fn new() -> Self {
        Self {
            length: 0,
            ptr: QueuePtr::Empty,
        }
    }

    pub fn enqueue(&mut self, value_in: T) {
        self.length += 1;
        match &self.ptr {
            QueuePtr::Empty => {
                self.ptr = QueuePtr::Unit { value: value_in };
            }

            QueuePtr::Unit { value } => {
                let tail: *mut QNode<T> = Box::into_raw(Box::new(QNode::Tail { value: value_in }));
                self.ptr = QueuePtr::Multi {
                    head_value: value.clone(),
                    head_next: tail,
                    tail,
                }
            }

            QueuePtr::Multi {
                head_value,
                head_next,
                tail,
            } => {
                let new_tail: *mut QNode<T> =
                    Box::into_raw(Box::new(QNode::Tail { value: value_in }));
                let old_tail = QNode::Body {
                    value: unsafe { (**tail).value() },
                    next: new_tail,
                };
                unsafe {
                    (**tail) = old_tail;
                }
                self.ptr = QueuePtr::Multi {
                    head_value: head_value.clone(),
                    head_next: *head_next,
                    tail: new_tail,
                };
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        match &self.ptr {
            QueuePtr::Empty => None,

            QueuePtr::Unit { value } => {
                self.length -= 1;

                let out = Some(value.clone());
                self.ptr = QueuePtr::Empty;
                out
            }

            QueuePtr::Multi {
                head_value,
                head_next,
                tail,
            } => {
                self.length -= 1;
                let cloned_head = (head_value).clone();
                let new_head = *head_next;
                self.ptr = if self.length == 1 {
                    unsafe {
                        drop(Box::from_raw(new_head));
                    }
                    QueuePtr::Unit {
                        value: head_value.clone(),
                    }
                } else if self.length == 0 {
                    println!("{new_head:?}0");
                    QueuePtr::Empty
                } else {
                    let next_head = unsafe { *Box::from_raw(new_head) };
                    QueuePtr::Multi {
                        head_value: next_head.value(),
                        head_next: next_head.next()?,
                        tail: *tail,
                    }
                };
                Some(cloned_head)
            }
        }
    }

    pub fn peek(&self) -> Option<T> {
        match &self.ptr {
            QueuePtr::Empty => None,

            QueuePtr::Unit { value } => Some(value.clone()),

            QueuePtr::Multi {
                head_value,
                head_next: _,
                tail: _,
            } => Some((*head_value).clone()),
        }
    }
}
