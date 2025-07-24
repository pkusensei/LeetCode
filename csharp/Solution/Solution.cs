using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumSquares(int n)
    {
        int[] dp = [.. Enumerable.Range(0, 1 + n)];
        for (int i = 1; i <= n; i++)
        {
            for (int root = 1; root * root <= i; root++)
            {
                dp[i] = int.Min(dp[i], 1 + dp[i - root * root]);
            }
        }
        return dp[n];
    }
}