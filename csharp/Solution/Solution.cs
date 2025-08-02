using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CombinationSum4(int[] nums, int target)
    {
        int[] dp = new int[1 + target];
        dp[0] = 1;
        for (int right = 1; right <= target; right++)
        {
            foreach (var num in nums)
            {
                int left = right - num;
                if (left >= 0) { dp[right] += dp[left]; }
            }
        }
        return dp[target];
    }
}