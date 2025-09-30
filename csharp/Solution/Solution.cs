using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindNumberOfLIS(int[] nums)
    {
        int n = nums.Length;
        int[] dp = new int[n];
        int[] count = new int[n];
        Array.Fill(dp, 1);
        Array.Fill(count, 1);
        int len = 1;
        for (int right = 0; right < n; right++)
        {
            for (int left = 0; left < right; left++)
            {
                if (nums[right] > nums[left])
                {
                    int curr = 1 + dp[left];
                    len = int.Max(len, curr);
                    if (curr > dp[right])
                    {
                        dp[right] = curr;
                        count[right] = count[left]; // continue this streak
                    }
                    else if (curr == dp[right])
                    {
                        count[right] += count[left]; // find another route
                    }
                }
            }
        }
        int res = 0;
        for (int i = 0; i < n; i++)
        {
            if (dp[i] == len) { res += count[i]; }
        }
        return res;
    }
}