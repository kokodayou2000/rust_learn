#[derive(PartialEq, Eq, Clone, Debug)]
#[warn(private_interfaces)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/// 2. 两数相加
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut carry = 0;
    let mut dummy = ListNode::new(0);
    let mut tail = &mut dummy;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let mut sum = carry;

        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }

        tail.next = Some(Box::new(ListNode::new(sum % 10)));
        tail = tail.next.as_mut().unwrap();
        carry = sum / 10;
    }

    dummy.next
}

// ====================== 测试代码 ======================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(mut nums: Vec<i32>) -> Option<Box<ListNode>> {
        nums.reverse();
        let mut head = None;
        for val in nums {
            head = Some(Box::new(ListNode { val, next: head }));
        }
        head
    }

    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }
        result
    }

    #[test]
    fn test_add_two_numbers() {
        // 示例 1
        let l1 = create_list(vec![2, 4, 3]);
        let l2 = create_list(vec![5, 6, 4]);
        assert_eq!(list_to_vec(add_two_numbers(l1, l2)), vec![7, 0, 8]);

        // 示例 2
        let l1 = create_list(vec![0]);
        let l2 = create_list(vec![0]);
        assert_eq!(list_to_vec(add_two_numbers(l1, l2)), vec![0]);

        // 示例 3
        let l1 = create_list(vec![9,9,9,9,9,9,9]);
        let l2 = create_list(vec![9,9,9,9]);
        assert_eq!(list_to_vec(add_two_numbers(l1, l2)), vec![8,9,9,9,0,0,0,1]);

        // 边界
        assert_eq!(add_two_numbers(None, None), None);
        assert_eq!(list_to_vec(add_two_numbers(create_list(vec![5]), None)), vec![5]);
    }
}