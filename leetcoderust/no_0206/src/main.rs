#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        // length = 0
        if head == None {
            return head;
        }
        let mut node1 = head.take().unwrap_or(Box::new(ListNode::new(0)));
        // length = 1
        if (*node1).next == None {
            return Some(node1);
        }
        // length >= 2
        let mut node2 = (*node1).next.take().unwrap_or(Box::new(ListNode::new(0)));
        (*node1).next = None;
        loop {
            if (*node2).next == None {
                node2.next = Some(node1);
                break;
            } else {
                let tmp = (*node2).next.take().unwrap_or(Box::new(ListNode::new(0)));
                node2.next = Some(node1);
                node1 = node2;
                node2 = tmp;
            }
        }
        Some(node2)
    }
}

struct Solution {}
fn main() {
    // assert_eq!(Solution::solve(vec![1, 7], 3), vec![3, 7]);
}
