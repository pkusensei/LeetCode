using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaxWidthRamp(int[] nums)
    {
        Stack<(int idx, int num)> stack = [];
        foreach (var (idx, num) in nums.Select((n, i) => (i, n)))
        {   // Keep leftmost smallest nums
            if (stack.Count == 0 || stack.Peek().num > num)
            {
                stack.Push((idx, num));
            }
        }
        var res = 0;
        foreach (var (right, num) in nums.Select((n, i) => (i, n)).Reverse())
        {
            while (stack.TryPeek(out var item) && item.num <= num)
            {   // search for smaller num as left as possible
                stack.Pop();
                res = Math.Max(res, right - item.idx);
            }
        }
        return res;
    }
}
