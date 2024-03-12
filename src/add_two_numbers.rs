use std::cmp::max;

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>)
                       -> Option<Box<ListNode>> {

    let l1_len = get_ll_len(&l1);
    let l2_len = get_ll_len(&l2);
    let result_len = max(l1_len, l2_len) + 1;
    let mut digits_vector = vec![0; result_len as usize];

    let mut l1_current = &l1;
    let mut l2_current = &l2;

    let mut carry = 0;
    for idx in 0..digits_vector.len() {

        let mut digit = carry;
        if let Some(l1_node) = l1_current {
            digit += l1_node.val;
            l1_current = &l1_node.next;
        }
        if let Some(l2_node) = l2_current {
            digit += l2_node.val;
            l2_current = &l2_node.next;
        }

        if digit > 9 {
            carry = digit / 10;
            digit = digit % 10;
        } else {
            carry = 0;
        }

        digits_vector[idx] = digit;
    }

    let ll = create_ll_from_vec(digits_vector);
    return ll;
}

fn get_ll_len(l1: &Option<Box<ListNode>>) -> i32 {
    let mut current = l1;
    let mut length = 0;

    while let Some(some_current) = current {
        current = &some_current.next;
        length += 1;
    }

    return length;
}

fn create_ll_from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(vec[0]));
    let mut current = head.as_mut();

    for digit in &vec[1..vec.len() - 1] {
        let new_node = Box::new(ListNode::new(*digit));
        current.next = Some(new_node);
        current = current.next.as_mut().unwrap();
    }

    if *vec.last().unwrap() > 0 {
        let tail = Box::new(ListNode::new(*vec.last().unwrap()));
        current.next = Some(tail);
    }

    return Some(head);
}

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


