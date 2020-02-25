class ListNode {
  final int val;
  final ListNode next;

  ListNode(int x) {
    val = x;
    next = null;
  }
}

public class Solution {
  public static void main(String[] args) {

  }

  public ListNode detectCycle(ListNode head) {
    ListNode fast, slow;
    fast = head;
    slow = head;

    while (true) {
      if (fast != null && fast.next != null && fast.next.next != null) {
        fast = fast.next.next;
      } else {
        return null;
      }

      if (slow != null & slow.next != null) {
        slow = slow.next;
      }

      if (fast == slow) {
        break;
      }
    }

    fast = head;
    for (; fast != slow; ) {
      fast = fast.next;
      slow = slow.next;
    }
    return fast;
  }
}
