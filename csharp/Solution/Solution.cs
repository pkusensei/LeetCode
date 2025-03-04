using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public ListNode MergeNodes(ListNode head)
    {
        var left = head;
        var right = head.next;
        while (right is not null)
        {
            var val = 0;
            while (right is not null && right.val > 0)
            {
                val += right.val;
                right = right.next;
            }
            left.val = val;
            if (right.next is not null) { left.next = right; }
            else { left.next = null; }
            left = right;
            right = right.next;
        }
        return head;
    }
}
