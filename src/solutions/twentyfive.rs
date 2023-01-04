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

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    fn _reverse(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut h = head;
        let mut node = &mut h;
        for _ in 0..k {
            if let Some(n) = node {
                node = &mut n.next;
            } else {
                return h;
            }
        }
        let mut next = _reverse(node.take(), k);
        while let Some(mut new_h) = h {
            h = new_h.next;
            new_h.next = next;
            next = Some(new_h);
        }
        next
    }
    _reverse(head, k)
}

#[test]
pub fn test_reverse_k_group() {
    assert_eq!(Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(5)))
                }))
            }))
        }))
    })), reverse_k_group(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(5)))
                }))
            }))
        }))
    })), 2));

    assert_eq!(Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(5)))
                }))
            }))
        }))
    })), reverse_k_group(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(5)))
                }))
            }))
        }))
    })), 3));
}