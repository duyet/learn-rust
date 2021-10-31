mod p1_two_sum;
mod p9_palindrome_number;
mod p234_palindrome_linked_list;

fn main() {
    println!("two_sum");
    println!("{:?}", p1_two_sum::Solution::two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", p1_two_sum::Solution::two_sum_hash(vec![2, 7, 11, 15], 9));

    println!("palindrome_number");
    println!("{}", p9_palindrome_number::Solution::is_palindrome(121));

    println!("palindrome_linked_list");
    let head = Some(Box::new(p234_palindrome_linked_list::ListNode::new(1)));
    println!("{}", p234_palindrome_linked_list::Solution::is_palindrome(head));
}
