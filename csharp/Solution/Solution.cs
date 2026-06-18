using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxSumAfterPartitioning(int[] arr, int k)
    {
        int n = arr.Length;
        int[] dp = new int[1 + n];
        for (int end = 1; end <= n; end++)
        {
            int max = arr[end - 1];
            for (int len = 1; len <= k && end - len >= 0; len++)
            {
                max = int.Max(max, arr[end - len]);
                dp[end] = int.Max(dp[end], dp[end - len] + len * max);
            }
        }
        return dp[n];
    }
}
