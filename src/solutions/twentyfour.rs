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

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn _swap_two(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut n1) = head {
            if let Some(mut n2) = n1.next {
                n1.next = _swap_two(n2.next);
                n2.next = Some(n1);
                Some(n2)
            } else {
                Some(n1)
            }
        } else {
            None
        }
    }
    _swap_two(head)
}

#[test]
pub fn test_swap_pairs() {
    assert_eq!(Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3)))
            }))
        }))
    })), swap_pairs(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(4)))
            }))
        }))
    }))));
}