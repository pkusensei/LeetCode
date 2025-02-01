using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public ListNode SwapNodes(ListNode head, int k)
    {
        var curr = head;
        List<ListNode> nodes = [];
        while (curr is not null)
        {
            nodes.Add(curr); curr = curr.next;
        }
        var n = nodes.Count;
        (nodes[k - 1].val, nodes[n - k].val) = (nodes[n - k].val, nodes[k - 1].val);
        return head;
    }
}
