using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumberOfWays(int n, int x)
    {
        int[] dp = new int[1 + n];
        dp[0] = 1;
        for (int root = 1; root <= n; root++)
        {
            int diff = (int)Math.Pow(root, x);
            // Backwards to exusre each root is used exactly once
            // i.e only counts already reached states
            for (int right = n; right >= diff; right -= 1)
            {
                dp[right] += dp[right - diff];
                dp[right] %= 1_000_000_007;
            }
        }
        return dp[n];
    }
}