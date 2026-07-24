using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumRollsToTarget(int n, int k, int target)
    {
        const int M = 1_000_000_007;
        int[] dp = new int[1 + target];
        dp[0] = 1;
        for (int _ = 1; _ <= n; _++)
        {
            int[] curr = new int[1 + target];
            for (int prev = 0; prev <= target; prev++)
            {
                for (int d = 1; d <= k && prev + d <= target; d++)
                {
                    curr[prev + d] = (curr[prev + d] + dp[prev]) % M;
                }
            }
            dp = curr;
        }
        return dp[target];
        return Dfs(n, target);

        int Dfs(int n, int total)
        {
            if (n == 0) { return total == 0 ? 1 : 0; }
            if (total < 0) { return 0; }
            int res = 0;
            for (int i = 1; i <= k; i++)
            {
                res = (res + Dfs(n - 1, total - i)) % M;
            }
            return res;
        }
    }
}
