using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MincostTickets(int[] days, int[] costs)
    {
        ReadOnlySpan<int> diffs = [1, 7, 30];
        int max = days[^1];
        int[] dp = new int[1 + max];
        Array.Fill(dp, int.MaxValue, 1, max);
        for (int d = 1; d <= max; d++)
        {
            if (Array.BinarySearch(days, d) >= 0)
            {
                for (int i = 0; i < 3; i++)
                {
                    int prev = int.Max(d - diffs[i], 0);
                    dp[d] = int.Min(dp[d], dp[prev] + costs[i]);
                }
            }
            else
            {
                dp[d] = dp[d - 1];
            }
        }
        return dp[^1];
    }
}
