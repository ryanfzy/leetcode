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

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
    _merge(list1, list2)
}

#[test]
pub fn test_merge_two_lists() {
    assert_eq!(Some(Box::new(ListNode::new(0))), merge_two_lists(None, Some(Box::new(ListNode::new(0)))));
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
                            next: None
                        }))
                    }))
                }))
            }))
        }))
    })), merge_two_lists(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
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
    }))));
}