#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }

 pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    fn _remove_from_end(head: Option<Box<ListNode>>, n: &mut i32) -> Option<Box<ListNode>> {
        if let Some(mut h) = head {
            h.next = _remove_from_end(h.next, n);
            *n -= 1;
            if *n == 0 {
                return h.next;
            }
            return Some(h);
        }
        None
    }
    let mut i = n;
    if let Some(r) = _remove_from_end(head, &mut i) {
        i -= 1;
        if i == 0 {
            return r.next;
        }
        return Some(r);
    }
    None
 }

 #[test]
 pub fn test_remove_nth_from_end() {
    let input = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: None
                    }))
                }))
            }))
        }))
    }));
    let expected = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 5,
                    next: None
                }))
            }))
        }))
    }));
    assert_eq!(expected, remove_nth_from_end(input, 2));

    assert_eq!(None, remove_nth_from_end(Some(Box::new(ListNode {
        val: 1,
        next: None
    })), 1));

    assert_eq!(Some(Box::new(ListNode {
        val: 1,
        next: None
    })), remove_nth_from_end(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: None
        }))
    })), 1));
 }