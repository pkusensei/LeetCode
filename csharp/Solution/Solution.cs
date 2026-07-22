using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int StoneGameII(int[] piles)
    {
        int n = piles.Length;
        if (n <= 2) { return piles.Sum(); }
        int[] suffix = [.. piles];
        for (int i = n - 2; i >= 0; i -= 1)
        {
            suffix[i] += suffix[1 + i];
        }
        int[,] dp = new int[n, n];
        for (int i = 0; i < n; i++)
        {
            for (int m = 1; m < n; m++)
            {
                if (i + 2 * m >= n) { dp[i, m] = suffix[i]; }
            }
        }
        for (int idx = n - 1; idx >= 0; idx -= 1)
        {
            for (int m = 1; m < n; m += 1)
            {
                for (int x = 1; idx + x < n && x <= 2 * m; x++)
                {
                    int oppo = dp[idx + x, int.Max(x, m)];
                    if (oppo > 0) { dp[idx, m] = int.Max(dp[idx, m], suffix[idx] - oppo); }
                }
            }
        }
        return dp[0, 1];
        int[,] memo = new int[n, n];
        return Dfs(0, 1);

        int Dfs(int idx, int m)
        {
            if (idx + 2 * m >= n) { return suffix[idx]; }
            if (memo[idx, m] > 0) { return memo[idx, m]; }
            int res = 0;
            for (int x = 1; x <= 2 * m; x++)
            {
                int oppo = Dfs(idx + x, int.Max(x, m));
                res = int.Max(res, suffix[idx] - oppo);
            }
            memo[idx, m] = res;
            return memo[idx, m];
        }
    }
}
