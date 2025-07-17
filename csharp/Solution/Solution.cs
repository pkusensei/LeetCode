using System.Text;
using Solution.LList;
// using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public void ReorderList(ListNode head)
    {
        if (head is null || head.next is null) { return; }
        var slow = head;
        var fast = head;
        while (fast.next is not null && fast.next.next is not null)
        {
            slow = slow.next;
            fast = fast.next.next;
        }
        var tail = Reverse(slow.next);
        slow.next = null;
        while (tail is not null)
        {
            var n1 = head.next;
            var n2 = tail.next;
            head.next = tail;
            tail.next = n1;
            head = n1;
            tail = n2;
        }

        static ListNode Reverse(ListNode head)
        {
            ListNode prev = null;
            ListNode curr = head;
            while (curr is not null)
            {
                var temp = curr.next;
                curr.next = prev;
                prev = curr;
                curr = temp;
            }
            return prev;
        }
    }
}