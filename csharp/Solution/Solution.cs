using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinimumTotal(IList<IList<int>> triangle)
    {
        int[] dp = [0];
        foreach (var row in triangle)
        {
            int[] curr = new int[row.Count];
            Array.Fill(curr, int.MaxValue);
            for (int i = 0; i < curr.Length; i++)
            {
                if (0 <= i - 1 && i - 1 < dp.Length) { curr[i] = int.Min(curr[i], row[i] + dp[i - 1]); }
                if (0 <= i && i < dp.Length) { curr[i] = int.Min(curr[i], row[i] + dp[i]); }
            }
            dp = curr;
        }
        return dp.Min();
    }
}