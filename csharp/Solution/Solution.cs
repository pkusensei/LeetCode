using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int LenLongestFibSubseq(int[] arr)
    {
        Dictionary<int, int> dict = arr.Select((v, i) => (v, i)).ToDictionary();
        int n = arr.Length;
        int[,] dp = new int[n, n];
        for (int i1 = 0; i1 < n; i1++)
        {
            for (int i2 = 0; i2 < n; i2++)
            {
                dp[i1, i2] = 1;
            }
        }
        int res = 0;
        foreach (var (i1, v1) in arr.Select((v, i) => (i, v)))
        {
            foreach (var (i2, v2) in arr.Select((v, i) => (i, v)).Skip(1 + i1))
            {
                if (dict.TryGetValue(v1 + v2, out var i3))
                {
                    dp[i2, i3] = Math.Max(dp[i2, i3], Math.Max(3, 1 + dp[i1, i2]));
                    res = Math.Max(res, dp[i2, i3]);
                }
            }
        }
        return res >= 3 ? res : 0;
    }
}

