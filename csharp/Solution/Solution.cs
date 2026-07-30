using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode RemoveZeroSumSublists(ListNode head)
    {
        ListNode dummy = new(0, head);
        ListNode curr = dummy;
        Dictionary<int, ListNode> prefix = [];
        int sum = 0;
        while (curr is not null)
        {
            sum += curr.val;
            prefix[sum] = curr;
            curr = curr.next;
        }
        sum = 0;
        curr = dummy;
        while (curr is not null)
        {
            sum += curr.val;
            if (prefix.TryGetValue(sum, out var node))
            {
                curr.next = node.next;
            }
            curr = curr.next;
        }
        return dummy.next;
    }
}
