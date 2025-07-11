using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode DeleteDuplicates(ListNode head)
    {
        if (head is null) { return null; }
        ListNode dummy = new(101, head);
        ListNode prev = dummy;
        var slow = head;
        var fast = head.next;
        while (fast is not null)
        {
            if (slow.val != fast.val)
            {
                prev = slow;
                slow = slow.next;
                fast = fast.next;
            }
            else
            {
                while (fast is not null && fast.val == slow.val)
                {
                    fast = fast.next;
                }
                prev.next = fast;
                slow = fast;
                fast = fast?.next;
            }
        }
        return dummy.next;
    }
}