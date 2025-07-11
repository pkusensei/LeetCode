using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int ClimbStairs(int n)
    {
        if (n <= 1) { return 1; }
        int dp0 = 1;
        int dp1 = 1;
        int dp2 = 0;
        for (int i = 2; i <= n; i++)
        {
            dp2 = dp0 + dp1;
            (dp0, dp1) = (dp1, dp2);
        }
        return dp2;
    }
}