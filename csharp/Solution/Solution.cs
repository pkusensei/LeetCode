using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public ListNode DeleteMiddle(ListNode head)
    {
        var slow = head;
        var fast = head;
        ListNode dummy = new(0, head);
        ListNode prev = dummy;
        while (fast is not null && fast.next is not null)
        {
            prev = slow;
            slow = slow.next;
            fast = fast.next.next;
        }
        prev.next = slow.next;
        return dummy.next;
    }
}
