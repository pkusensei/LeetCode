using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int NumComponents(ListNode head, int[] nums)
    {
        if (head is null) { return 0; }
        var set = nums.ToHashSet();
        var curr = head;
        var res = 0;
        while (curr is not null)
        {
            while (curr is not null && !set.Contains(curr.val))
            {
                curr = curr.next;
            }
            if (curr is not null) { res += 1; }
            while (curr is not null && set.Contains(curr.val))
            {
                curr = curr.next;
            }
        }
        return res;
    }
}
