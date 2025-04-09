using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public ListNode DoubleIt(ListNode head)
    {
        ListNode dummy = new(0, head);
        int carry = Dfs(head);
        dummy.val = carry;
        if (carry == 0) { return head; }
        else { return dummy; }

        static int Dfs(ListNode node)
        {
            if (node is null) { return 0; }
            int carry = Dfs(node.next);
            int val = carry + 2 * node.val;
            node.val = val % 10;
            return val / 10;
        }
    }
}
