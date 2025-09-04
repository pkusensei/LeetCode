using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountArrangement(int n)
    {
        int sz = 1 << n;
        int[] dp = new int[sz];
        dp[0] = 1;
        for (int mask = 1; mask < sz; mask++)
        {
            int pos = int.PopCount(mask);
            for (int bit = 0; bit < n; bit++)
            {
                if (((mask >> bit) & 1) == 1 && (pos % (1 + bit) == 0 || (1 + bit) % pos == 0))
                {
                    dp[mask] += dp[mask ^ (1 << bit)];
                }
            }
        }
        return dp[^1];
        return Dfs(n, 0, 1);

        static int Dfs(int n, int mask, int depth)
        {
            if (int.PopCount(mask) == n) { return 1; }
            int res = 0;
            for (int bit = 0; bit < n; bit++)
            {
                if (((mask >> bit) & 1) == 0
                    && ((1 + bit) % depth == 0 || depth % (1 + bit) == 0))
                {
                    res += Dfs(n, mask | (1 << bit), 1 + depth);
                }
            }
            return res;
        }
    }
}