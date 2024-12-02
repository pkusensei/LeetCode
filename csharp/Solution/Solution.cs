using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int StoneGameII(int[] piles)
    {
        if (piles is null || piles.Length == 0) { return 0; }
        if (piles.Length == 2) { return piles.Sum(); }
        for (int i = piles.Length - 2; i >= 0; i -= 1)
        {
            piles[i] += piles[1 + i];
        }
        var dp = new int[piles.Length, piles.Length];
        return Dfs(0, 1);

        int Dfs(int idx, int m)
        {
            // able to take them all
            if (idx + 2 * m >= piles.Length) { return piles[idx]; }
            if (dp[idx, m] > 0) { return dp[idx, m]; }
            var res = int.MaxValue;
            for (int i = 1; i <= 2 * m; i++)
            {
                // 1 <= i <= 2*m
                // m is reset to max(i, m)
                res = Math.Min(res, Dfs(i + idx, Math.Max(i, m)));
            }
            dp[idx, m] = piles[idx] - res;
            return dp[idx, m];
        }
    }
}
