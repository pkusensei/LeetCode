using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsInterleave(string s1, string s2, string s3)
    {
        int n1 = s1.Length;
        int n2 = s2.Length;
        if (n1 + n2 != s3.Length) { return false; }
        bool[,] dp = new bool[1 + n1, 1 + n2];
        dp[0, 0] = true;
        for (int i = 0; i < n1; i++)
        {
            dp[1 + i, 0] = dp[i, 0] && s1[i] == s3[i];
        }
        for (int i = 0; i < n2; i++)
        {
            dp[0, 1 + i] = dp[0, i] && s2[i] == s3[i];
        }
        for (int i1 = 1; i1 <= n1; i1++)
        {
            for (int i2 = 1; i2 <= n2; i2++)
            {
                if (s1[i1 - 1] == s3[i1 + i2 - 1]) { dp[i1, i2] |= dp[i1 - 1, i2]; }
                if (s2[i2 - 1] == s3[i1 + i2 - 1]) { dp[i1, i2] |= dp[i1, i2 - 1]; }
            }
        }
        return dp[n1, n2];
    }

    public bool With1dDp(string s1, string s2, string s3)
    {
        int n1 = s1.Length;
        int n2 = s2.Length;
        if (n1 + n2 != s3.Length) { return false; }
        bool[] dp = new bool[1 + n2];
        dp[0] = true;
        for (int i = 1; i <= n2; i++)
        {
            dp[i] = dp[i - 1] && s2[i - 1] == s3[i - 1];
        }
        for (int i1 = 1; i1 <= n1; i1++)
        {
            dp[0] = dp[0] && s1[i1 - 1] == s3[i1 - 1];
            for (int i2 = 1; i2 <= n2; i2++)
            {
                dp[i2] = (dp[i2] && s1[i1 - 1] == s3[i1 + i2 - 1])
                    || (dp[i2 - 1] && s2[i2 - 1] == s3[i1 + i2 - 1]);
            }
        }
        return dp[n2];
    }
}