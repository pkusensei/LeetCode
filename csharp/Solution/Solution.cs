using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaxAbsoluteSum(int[] nums)
    {
        int min = 0;
        int max = 0;
        int res = 0;
        foreach (var num in nums)
        {
            min = Math.Min(num, min + num);
            max = Math.Max(num, max + num);
            res = Math.Max(res, Math.Max(Math.Abs(min), Math.Abs(max)));
        }
        return res;
    }
}

