using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaximumLength(int[] nums, int k)
    {
        int res = 0;
        int[] dp = new int[k];
        for (int val = 0; val < k; val++)
        {
            Array.Fill(dp, 0);
            foreach (var num in nums)
            {
                int prev = (k + val - num % k) % k;
                dp[num % k] = 1 + dp[prev];
                res = int.Max(res, dp[num % k]);
            }
        }
        return res;
    }
}