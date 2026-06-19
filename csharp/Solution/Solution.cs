using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LastStoneWeightII(int[] stones)
    {
        int n = stones.Length;
        int sum = stones.Sum();
        int[] dp = new int[1 + sum];
        for (int s = 0; s <= sum; s++)
        {
            dp[s] = int.Abs(sum - 2 * s);
        }
        foreach (var item in stones)
        {
            int[] curr = new int[1 + sum];
            for (int s = 0; s <= sum; s++)
            {
                int pick = s + item <= sum ? dp[s + item] : int.MaxValue;
                curr[s] = int.Min(dp[s], pick);
            }
            dp = curr;
        }
        return dp[0];

        int Dfs(int i, int curr)
        {
            if (i >= n || 2 * curr >= sum) { return int.Abs(sum - 2 * curr); }
            int skip = Dfs(1 + i, curr);
            int pick = Dfs(1 + i, curr + stones[i]);
            return int.Min(skip, pick);
        }
    }
}
