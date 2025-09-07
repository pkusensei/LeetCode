using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CheckRecord(int n)
    {
        const int M = 1_000_000_007;
        int[,,] memo = new int[1 + n, 2, 3];
        for (int p = 0; p <= n; p++)
        {
            for (int a = 0; a < 2; a++)
            {
                for (int l_ = 0; l_ < 3; l_++)
                {
                    memo[p, a, l_] = -1;
                }
            }
        }
        return Dfs(n, 1, 2);

        int Dfs(int n, int a, int streak)
        {
            if (a < 0 || streak < 0) { return 0; }
            if (n == 0) { return 1; }
            if (memo[n, a, streak] > -1) { return memo[n, a, streak]; }
            int res = (Dfs(n - 1, a, 2) + Dfs(n - 1, a - 1, 2)) % M + Dfs(n - 1, a, streak - 1);
            res %= M;
            memo[n, a, streak] = res;
            return res;
        }
    }

    public int WithTwoPhases(int n)
    {
        const int M = 1_000_000_007;
        long[] dp = new long[n + 1];

        // Phase 1) No absences
        dp[0] = 1;
        if (n >= 1) dp[1] = 2;
        if (n >= 2) dp[2] = 4; // PP PL LP LL
        for (int i = 3; i <= n; i++)
        {
            dp[i] = (dp[i - 1] + dp[i - 2] + dp[i - 3]) % M;
            // To build seq of length i:
            // Add P to any [i-1]
            // Add PL to any [i-2]
            // Add PLL to any [i-3]
        }
        long res = dp[n]; // Accumulates counts with no absence

        // Phase 2) With one absence
        for (int i = 0; i < n; i++)
        {
            // [0..i) => dp[i]
            // [i]=A
            // [1+i..] => dp[n-i-1]
            res = (res + dp[i] * dp[n - i - 1]) % M;
        }
        return (int)res;
    }
}
