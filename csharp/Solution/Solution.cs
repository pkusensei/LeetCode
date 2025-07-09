using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Jump(int[] nums)
    {
        int[] dp = new int[nums.Length];
        Array.Fill(dp, int.MaxValue);
        dp[0] = 0;
        for (int i = 0; i < nums.Length; i++)
        {
            for (int d = 1; d <= nums[i] && i + d < nums.Length; d++)
            {
                dp[i + d] = Math.Min(dp[i + d], 1 + dp[i]);
            }
        }
        return dp.Last();
    }
}
