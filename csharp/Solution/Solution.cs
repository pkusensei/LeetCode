using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public ListNode RemoveZeroSumSublists(ListNode head)
    {
        ListNode dummy = new(0, head);
        Dictionary<int, ListNode> dict = [];
        int sum = 0;
        var curr = dummy;
        while (curr is not null)
        {
            sum += curr.val;
            dict[sum] = curr;
            curr = curr.next;
        }
        sum = 0;
        curr = dummy;
        while (curr is not null)
        {
            sum += curr.val;
            if (dict.TryGetValue(sum, out var node))
            {
                curr.next = node.next;
            }
            curr = curr.next;
        }
        return dummy.next;
    }
}
