using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumDistinct(string s, string t)
    {
        int n1 = s.Length;
        int n2 = t.Length;
        if (n1 < n2) { return 0; }
        int[,] dp = new int[1 + n1, 1 + n2];
        for (int i = 0; i <= n1; i++)
        {
            dp[i, 0] = 1;
        }
        // for (int i1 = 1; i1 <= n1; i1++)
        // {
        //     for (int i2 = 1; i2 <= n2; i2++)
        //     {
        //         dp[i1, i2] = dp[i1 - 1, i2];
        //         if (s[i1 - 1] == t[i2 - 1]) { dp[i1, i2] += dp[i1 - 1, i2 - 1]; }
        //     }
        // }
        for (int i1 = 0; i1 < n1; i1++)
        {
            for (int i2 = 0; i2 < n2; i2++)
            {
                dp[1 + i1, 1 + i2] = dp[i1, 1 + i2];
                if (s[i1] == t[i2]) { dp[1 + i1, 1 + i2] += dp[i1, i2]; }
            }
        }
        return dp[n1, n2];
    }
}