using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public string ShortestCommonSupersequence(string str1, string str2)
    {
        (int n1, int n2) = (str1.Length, str2.Length);
        int[,] dp = new int[1 + n1, 1 + n2];
        foreach (var (i1, c1) in str1.Select((c, i) => (i, c)))
        {
            foreach (var (i2, c2) in str2.Select((c, i) => (i, c)))
            {
                if (c1 == c2) { dp[1 + i1, 1 + i2] = 1 + dp[i1, i2]; }
                else { dp[1 + i1, 1 + i2] = Math.Max(dp[1 + i1, i2], dp[i1, 1 + i2]); }
            }
        }
        StringBuilder sb = new();
        (int ii1, int ii2) = (n1, n2);
        while (ii1 > 0 && ii2 > 0)
        {
            if (str1[ii1 - 1] == str2[ii2 - 1])
            {
                sb.Append(str1[ii1 - 1]);
                ii1 -= 1;
                ii2 -= 1;
            }
            else if (dp[ii1 - 1, ii2] > dp[ii1, ii2 - 1])
            {
                sb.Append(str1[ii1 - 1]);
                ii1 -= 1;
            }
            else
            {
                sb.Append(str2[ii2 - 1]);
                ii2 -= 1;
            }
        }
        for (int i = ii1 - 1; i >= 0; i -= 1) { sb.Append(str1[i]); }
        for (int i = ii2 - 1; i >= 0; i -= 1) { sb.Append(str2[i]); }
        return new([.. sb.ToString().Reverse()]);
    }
}
