pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        let mut stack = vec![];
        let mut node = head;

        while node.is_some() {
            stack.push(node.as_ref().unwrap().val as i32); 
            node = node.as_ref().unwrap().next;
        }

        while head.is_some() {
            if head.as_ref().unwrap().val != stack.pop().unwrap() {
                return false;
            }
            head = head.as_ref().unwrap().next;
        }

        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_palindrome(None), true);
    assert_eq!(
        Solution::is_palindrome(Some(Box::new(ListNode::new(1)))),
        true
    );

    let node = Some(Box::new(ListNode::new(1)));
    assert_eq!(
        Solution::is_palindrome(node),
        false
    );
}
