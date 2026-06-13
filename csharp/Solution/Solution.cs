using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxUncrossedLines(int[] nums1, int[] nums2)
    {
        int n1 = nums1.Length;
        int n2 = nums2.Length;
        int[,] dp = new int[1 + n1, 1 + n2];
        for (int i1 = 0; i1 < n1; i1++)
        {
            for (int i2 = 0; i2 < n2; i2++)
            {
                dp[1 + i1, 1 + i2] = nums1[i1] == nums2[i2] ? 1 + dp[i1, i2] : int.Max(dp[1 + i1, i2], dp[i1, 1 + i2]);
            }
        }
        return dp[n1, n2];
    }
}
