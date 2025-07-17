using System.Text;
using Solution.LList;
// using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode DetectCycle(ListNode head)
    {
        if (head is null) { return null; }
        bool cycle = false;
        var slow = head;
        var fast = head;
        while (fast is not null && fast.next is not null)
        {
            slow = slow.next;
            fast = fast.next.next;
            if (slow == fast)
            {
                cycle = true;
                break;
            }
        }
        if (!cycle) { return null; }
        slow = head;
        while (slow != fast)
        {
            slow = slow.next;
            fast = fast.next;
        }
        return slow;
    }
}