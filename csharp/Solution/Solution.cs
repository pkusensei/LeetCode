using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public long MostPoints(int[][] questions)
    {
        int n = questions.Length;
        long[] dp = new long[n];
        for (int i = n - 1; i >= 0; i -= 1)
        {
            int pts = questions[i][0];
            int pow = questions[i][1];
            long skip = 1 + i >= n ? 0 : dp[1 + i];
            int next = i + pow + 1;
            long take = (long)pts + (next >= n ? 0 : dp[next]);
            dp[i] = Math.Max(skip, take);
        }
        return dp[0];
    }
}
