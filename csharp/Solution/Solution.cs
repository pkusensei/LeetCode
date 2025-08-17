using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public double New21Game(int n, int k, int maxPts)
    {
        List<double> dp = new(1 + n) { 1.0 };
        double curr = k > 0 ? 1.0 : 0.0;
        for (int score = 1; score <= n; score++)
        {
            dp.Add(curr / maxPts);
            if (score < k) { curr += dp[score]; }
            if (score >= maxPts && score - maxPts < k) { curr -= dp[score - maxPts]; }
        }
        return dp[k..].Sum();
    }
}