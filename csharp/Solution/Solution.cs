using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MincostTickets(int[] days, int[] costs)
    {
        var n = 1 + days.Last();
        var dp = new int[n];
        HashSet<int> set = [.. days];
        for (int i = 1; i < n; i++)
        {
            if (set.Contains(i))
            {
                dp[i] = dp[i - 1] + costs[0];
                dp[i] = Math.Min(dp[i], costs[1] + (i >= 7 ? dp[i - 7] : 0));
                dp[i] = Math.Min(dp[i], costs[2] + (i >= 30 ? dp[i - 30] : 0));
            }
            else
            {
                dp[i] = dp[i - 1];
            }
        }
        return dp.Last();
    }

    public int TopDown(int[] days, int[] costs)
    {
        var n = 1 + days.Last();
        var dp = new int[n];
        Array.Fill(dp, -1);
        HashSet<int> set = [.. days];
        return Dfs(dp, 1);

        int Dfs(int[] dp, int curr)
        {
            if (curr > days.Last()) { return 0; }
            if (!set.Contains(curr)) { return Dfs(dp, 1 + curr); }
            if (dp[curr] > -1) { return dp[curr]; }
            var a = costs[0] + Dfs(dp, 1 + curr);
            var b = costs[1] + Dfs(dp, 7 + curr);
            var c = costs[2] + Dfs(dp, 30 + curr);
            dp[curr] = Math.Min(a, Math.Min(b, c));
            return dp[curr];
        }
    }
}