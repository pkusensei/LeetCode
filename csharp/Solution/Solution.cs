using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public ListNode MiddleNode(ListNode head)
    {
        (var slow, var fast) = (head, head);
        while (slow is not null && fast is not null && fast.next is not null)
        {
            slow = slow.next;
            fast = fast.next.next;
        }
        return slow;
    }
}
