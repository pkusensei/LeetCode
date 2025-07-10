using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxSubArray(int[] nums)
    {
        int res = int.MinValue;
        int curr = 0;
        foreach (var num in nums)
        {
            curr = int.Max(num, curr + num);
            res = int.Max(res, curr);
        }
        return res;
    }
}