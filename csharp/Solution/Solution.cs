using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinDistance(string word1, string word2)
    {
        int n1 = word1.Length;
        int n2 = word2.Length;
        int[,] dp = new int[1 + n1, 1 + n2];
        int lcs = 0;
        for (int i1 = 0; i1 < n1; i1++)
        {
            for (int i2 = 0; i2 < n2; i2++)
            {
                if (word1[i1] == word2[i2])
                {
                    dp[1 + i1, 1 + i2] = int.Max(dp[1 + i1, 1 + i2], 1 + dp[i1, i2]);
                }
                else
                {
                    dp[1 + i1, 1 + i2] = int.Max(dp[1 + i1, i2], dp[i1, 1 + i2]);
                }
                lcs = int.Max(lcs, dp[1 + i1, 1 + i2]);
            }
        }
        return n1 + n2 - 2 * lcs;
    }
}
