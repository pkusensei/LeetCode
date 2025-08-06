using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool CanCross(int[] stones)
    {
        int n = stones.Length;
        List<HashSet<int>> dp = [[1]];
        for (int right = 1; right < n; right++)
        {
            HashSet<int> curr = [];
            for (int left = 0; left < right; left++)
            {
                int diff = stones[right] - stones[left];
                if (dp[left].Contains(diff)) { curr.UnionWith([diff - 1, diff, diff + 1]); }
            }
            dp.Add(curr);
        }
        return dp.Count >= n && dp[n - 1].Count > 0;
    }
}
