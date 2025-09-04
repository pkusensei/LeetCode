using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Change(int amount, int[] coins)
    {
        long[] dp = new long[1 + amount];
        dp[0] = 1;
        foreach (var c in coins)
        {
            for (int i = 0; i <= amount; i++)
            {
                if (c + i <= amount)
                {
                    dp[c + i] += dp[i];
                }
            }
        }
        return (int)dp[^1];
    }
}