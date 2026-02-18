using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int ProfitableSchemes(int n, int minProfit, int[] group, int[] profit)
    {
        const int M = 1_000_000_007;
        int len = group.Length;
        int[,] dp = new int[1 + n, 1 + minProfit];
        for (int i = 0; i <= n; i++)
        {
            dp[i, 0] = 1;
        }
        foreach (var (gro, pro) in group.Zip(profit))
        {
            for (int c = n; c >= gro; c -= 1)
            {
                for (int p = minProfit; p >= 0; p -= 1)
                {
                    dp[c, p] = (dp[c, p] + dp[c - gro, int.Max(0, p - pro)]) % M;
                }
            }
        }
        return dp[n, minProfit];
        // int[,] prev = new int[1 + n, 1 + minProfit];
        // for (int i = 0; i <= n; i++)
        // {
        //     prev[i, 0] = 1;
        // }
        // foreach (var (gro, pro) in group.Zip(profit))
        // {
        //     int[,] curr = new int[1 + n, 1 + minProfit];
        //     for (int c = 0; c <= n; c++)
        //     {
        //         for (int p = 0; p <= minProfit; p++)
        //         {
        //             curr[c, p] = prev[c, p];
        //             if (c - gro >= 0)
        //             {
        //                 curr[c, p] += prev[c - gro, int.Max(0, p - pro)];
        //                 curr[c, p] %= M;
        //             }
        //         }
        //     }
        //     prev = curr;
        // }
        // return prev[n, minProfit];

        // int[,,] dp = new int[1 + len, 1 + n, 1 + minProfit];
        // for (int i = 0; i <= n; i++)
        // {
        //     dp[0, i, 0] = 1;
        // }
        // for (int idx = 0; idx < len; idx += 1)
        // {
        //     for (int c = 0; c <= n; c++)
        //     {
        //         for (int p = 0; p <= minProfit; p++)
        //         {
        //             dp[1 + idx, c, p] = dp[idx, c, p];
        //             if (c - group[idx] >= 0)
        //             {
        //                 dp[1 + idx, c, p] += dp[idx, c - group[idx], int.Max(0, p - profit[idx])];
        //                 dp[1 + idx, c, p] %= M;
        //             }
        //         }
        //     }
        // }
        // return dp[len, n, minProfit];

        // return Dfs(0, n, minProfit);

        // int Dfs(int idx, int count, int min_profit)
        // {
        //     if (idx >= len) { return min_profit <= 0 ? 1 : 0; }
        //     if (count < 0) { return 0; }
        //     return Dfs(1 + idx, count, min_profit)
        //         + Dfs(1 + idx, count - group[idx], min_profit - profit[idx]);
        // }
    }
}