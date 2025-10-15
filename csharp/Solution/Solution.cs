using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool CanPartitionKSubsets(int[] nums, int k)
    {
        int n = nums.Length;
        int sum = nums.Sum();
        if (sum % k > 0) { return false; }
        int target = sum / k;
        int[] dp = new int[1 << n];
        Array.Fill(dp, -1);
        dp[0] = 0;
        for (int mask = 0; mask < (1 << n); mask++)
        {
            if (dp[mask] < 0) { continue; }
            for (int i = 0; i < n; i++)
            {
                if (((mask >> i) & 1) == 1) { continue; }
                int curr = mask | (1 << i);
                if (dp[mask] + nums[i] <= target && dp[curr] == -1)
                {
                    // Implicity switch buckets
                    // i.e when this one is full, start a new one
                    dp[curr] = (dp[mask] + nums[i]) % target;
                }
            }
        }
        return dp[^1] == 0;
    }
}
