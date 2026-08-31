using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] NodesBetweenCriticalPoints(ListNode head)
    {
        int prev_idx = -1;
        int idx = 0;
        int first_idx = -1;
        int prev_val = -1;
        ListNode curr = head;
        int min = -1;
        int max = -1;
        while (curr is not null)
        {
            idx += 1;
            if (prev_val > 0 && curr.next is ListNode next_node &&
            (prev_val > curr.val && curr.val < next_node.val
            || prev_val < curr.val && curr.val > next_node.val)
            )
            {
                if (min == -1 && prev_idx > 0) { min = idx - prev_idx; }
                else { min = int.Min(min, idx - prev_idx); }
                if (first_idx == -1) { first_idx = idx; }
                else { max = int.Max(max, idx - first_idx); }
                prev_idx = idx;
            }
            prev_val = curr.val;
            curr = curr.next;
        }
        return [min, max];
    }
}

