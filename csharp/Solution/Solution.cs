using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<int> LargestDivisibleSubset(int[] nums)
    {
        Array.Sort(nums);
        int n = nums.Length;
        Span<int> dp = stackalloc int[n];
        Span<int> prev = stackalloc int[n];
        prev.Fill(-1);
        int max_idx = 0;
        for (int i1 = 1; i1 < n; i1++)
        {
            for (int i2 = 0; i2 < i1; i2++)
            {
                if (nums[i1] % nums[i2] == 0 && dp[i1] <= dp[i2])
                {
                    dp[i1] = 1 + dp[i2];
                    prev[i1] = i2;
                }
            }
            if (dp[i1] > dp[max_idx]) { max_idx = i1; }
        }
        var res = new List<int>();
        while (max_idx != -1)
        {
            res.Add(nums[max_idx]);
            max_idx = prev[max_idx];
        }
        res.Reverse();
        return res;
    }
}
