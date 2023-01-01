#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
 
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    fn _merge(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(x), Some(y)) => {
                if x.val <= y.val {
                    Some(Box::new(ListNode {
                        val: x.val,
                        next: _merge(x.next, Some(y))
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: y.val,
                        next: _merge(Some(x), y.next)
                    }))
                }
            },
            (Some(x), None) => {
                Some(x)
            },
            (None, Some(y)) => {
                Some(y)
            },
            _ => None
        }
    }
    if lists.len() == 0 {
        return None;
    }
    if let Some(mut result) = lists.pop() {
        while let Some(node) = lists.pop() {
            result = _merge(result, node);
        }
        return result;
    }
    None
}

pub fn merge_k_lists2(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut vals = vec![];
    for node in lists {
        if let Some(mut n) = node {
            vals.push(n.val);
            while let Some(next) = n.next {
                vals.push(next.val);
                n = next;
            }
        }
    }
    fn _build(vals: &[i32]) -> Option<Box<ListNode>> {
        if vals.len() == 0 {
            return None;
        } else if vals.len() == 1 {
            Some(Box::new(ListNode {
                val: vals[0],
                next:None
            }))
        } else {
            Some(Box::new(ListNode {
                val: vals[0],
                next: _build(&vals[1..])
            }))
        }
    }
    vals.sort();
    _build(&vals)
}

#[test]
pub fn test_merge_k_lists() {
    assert_eq!(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode {
                                    val: 6,
                                    next: None
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }))
    })), merge_k_lists(vec![Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 5,
                next: None
            }))
        }))
    })), Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None
            }))
        }))
    })), Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 6,
            next: None
        }))
    }))]));
}

#[test]
pub fn test_merge_k_lists2() {
    assert_eq!(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode {
                                    val: 6,
                                    next: None
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }))
    })), merge_k_lists2(vec![Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 5,
                next: None
            }))
        }))
    })), Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None
            }))
        }))
    })), Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 6,
            next: None
        }))
    }))]));
}