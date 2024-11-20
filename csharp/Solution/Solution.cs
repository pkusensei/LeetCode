using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int[] NextLargerNodes(ListNode head)
    {
        List<int> res = [];
        Stack<(int idx, int num)> nums = [];
        var curr = head;
        var idx = 0;
        while (curr is not null)
        {
            while (nums.TryPeek(out var v) && v.num < curr.val)
            {
                nums.Pop();
                res[v.idx] = curr.val;
            }
            nums.Push((idx, curr.val));
            res.Add(0);
            curr = curr.next;
            idx += 1;
        }
        return [.. res];
    }
}
