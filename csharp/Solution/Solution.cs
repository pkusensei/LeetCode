using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode DeleteMiddle(ListNode head)
    {
        ListNode dummy = new(0, head);
        ListNode prev = dummy;
        ListNode slow = head;
        ListNode fast = head;
        while (fast is not null && fast.next is not null)
        {
            fast = fast.next.next;
            prev = slow;
            slow = slow.next;
        }
        prev.next = slow.next;
        return dummy.next;
    }
}
