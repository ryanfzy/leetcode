// Definition for singly-linked list.
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

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn _add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(mut a), Some(b)) => {
                let n = a.val + b.val;
                if n > 9 {
                    if let Some(a_next) = a.next.as_mut() {
                        a_next.val += 1;
                    }
                    else {
                        a.next = Some(Box::new(ListNode::new(1)));
                    }
                }
                let mut r = ListNode::new(n % 10);
                r.next = _add(a.next, b.next);
                Some(Box::new(r))
            },
            (Some(mut a), None) => {
                let n = a.val;
                if n > 9 {
                    if let Some(a_next) = a.next.as_mut() {
                        a_next.val += 1;
                    }
                    else {
                        a.next = Some(Box::new(ListNode::new(1)));
                    }
                }
                let mut r = ListNode::new(n % 10);
                r.next = _add(a.next, None);
                Some(Box::new(r))
            },
            (None, Some(b)) => _add(Some(b), None),
            // can be further simplified to:
            // (Some(a), None) => _add(Some(a), Some(Box::new(ListNode::new(0)))),
            // (None, Some(b)) => _add(Some(Box::new(ListNode::new(0))), Some(b)),
            _ => None
        }
    }
    _add(l1, l2)
}

#[test]
fn test_add_two_numbers() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 3,
                next: None
            }))
        }))
    }));
    let l2 = Some(Box::new(ListNode {
        val: 5, 
        next: Some(Box::new(ListNode {
            val: 6, 
            next: Some(Box::new(ListNode {
                val: 4, 
                next: None
            }))
        }))
    }));
    let r = add_two_numbers(l1, l2);
    assert_eq!(r, Some(Box::new(ListNode {
        val: 7,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 8,
                next: None
            }))
        }))
    })));
}