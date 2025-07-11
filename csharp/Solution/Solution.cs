using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinDistance(string word1, string word2)
    {
        if (word1 == word2) { return 0; }
        int n1 = word1.Length;
        int n2 = word2.Length;
        if (n1 * n2 == 0) { return int.Max(n1, n2); }
        int[,] dp = new int[1 + n1, 1 + n2];
        for (int i1 = 0; i1 <= n1; i1++)
        {
            for (int i2 = 0; i2 <= n2; i2++)
            {
                if (i1 == 0) { dp[i1, i2] = i2; }
                else if (i2 == 0) { dp[i1, i2] = i1; }
                else if (word1[i1 - 1] == word2[i2 - 1])
                {
                    dp[i1, i2] = dp[i1 - 1, i2 - 1];
                }
                else
                {
                    dp[i1, i2] = 1 + int.Min(dp[i1, i2 - 1], // insert
                                     int.Min(dp[i1 - 1, i2], // delete
                                             dp[i1 - 1, i2 - 1])); // replace
                }
            }
        }
        return dp[n1, n2];
    }
}