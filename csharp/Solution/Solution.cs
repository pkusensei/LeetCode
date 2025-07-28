using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CoinChange(int[] coins, int amount)
    {
        if (amount < 0) { return 0; }
        int[] dp = new int[1 + amount];
        Array.Fill(dp, 1 + amount);
        dp[0] = 0;
        for (int right = 1; right <= amount; right++)
        {
            foreach (var c in coins)
            {
                int left = right - c;
                if (left >= 0) { dp[right] = int.Min(dp[right], 1 + dp[left]); }
            }
        }
        return dp[amount] > amount ? -1 : dp[amount];
    }
}
