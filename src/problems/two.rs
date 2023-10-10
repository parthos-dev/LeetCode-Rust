// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    #[inline]
    pub fn from_vec(arr: &Vec<i32>) -> Option<Box<ListNode>>{
        assert!(!arr.is_empty());
        let mut l1 = Some(Box::new(ListNode{ val: *arr.last().unwrap(), next: None }));
        for i in arr.iter().rev().skip(1) {
            l1 = Some(Box::new(ListNode{ val: *i, next: l1 }));
        }

        return l1;
    }
}

pub fn run() {
    // let (a, b) = (vec![1,2,3,4,5,6,7], vec![1,2,3,4]);
    let (a, b) = (vec![9,9,9,9,9,9,9], vec![9,9,9,9]);
    let l1 = ListNode::from_vec(&a);
    let l2 = ListNode::from_vec(&b);

    let out = add_two_numbers(l1, l2);
    println!("{:?}", out)
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let def = Box::new(ListNode{ val: 0, next: None });
    let mut iter1 = (&l1).as_ref();
    let mut iter2 = (&l2).as_ref();

    let mut sum_two: Option<Box<ListNode>> = None;
    let mut tail = &mut sum_two;

    let mut extra = 0;
    while iter1.is_some() || iter2.is_some() || extra > 0 {
        let sum = (iter1.unwrap_or(&def).val + iter2.unwrap_or(&def).val + extra);
        extra = if sum > 9 { 1 } else { 0 };

        *tail = Some(Box::new(ListNode{ val: sum % 10, next: None }));
        tail = &mut tail.as_mut().unwrap().next;

        if iter1.is_some() {
            iter1 = iter1.unwrap().next.as_ref();
        }
        if iter2.is_some(){
            iter2 = iter2.unwrap().next.as_ref();
        }
    }

    return sum_two;
}

