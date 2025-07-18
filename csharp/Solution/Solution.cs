using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode SortList(ListNode head)
    {
        return MergeSort(head);

        static ListNode MergeSort(ListNode head)
        {
            if (head is null || head.next is null) { return head; }
            var slow = head;
            var fast = head;
            while (fast.next is not null && fast.next.next is not null)
            {
                slow = slow.next;
                fast = fast.next.next;
            }
            var n2 = MergeSort(slow.next);
            slow.next = null;
            var n1 = MergeSort(head);
            ListNode dummy = new(int.MinValue);
            var curr = dummy;
            while (n1 is not null && n2 is not null)
            {
                if (n1.val <= n2.val)
                {
                    curr.next = n1;
                    n1 = n1.next;
                }
                else
                {
                    curr.next = n2;
                    n2 = n2.next;
                }
                curr = curr.next;
            }
            if (n1 is not null) { curr.next = n1; }
            if (n2 is not null) { curr.next = n2; }
            return dummy.next;
        }
    }
}