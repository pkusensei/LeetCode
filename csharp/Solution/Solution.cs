using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int VideoStitching(int[][] clips, int time)
    {
        int n = clips.Length;
        Array.Sort(clips, (a, b) => a[0] == b[0] ? a[1].CompareTo(b[1]) : a[0].CompareTo(b[0]));
        int[] dp = new int[1 + time];
        Array.Fill(dp, 1 + n);
        dp[0] = 0;
        foreach (var item in clips)
        {
            for (int i = 1 + item[0]; i <= int.Min(time, item[1]); i++)
            {
                dp[i] = int.Min(dp[i], 1 + dp[item[0]]);
            }
        }
        return dp[^1] > n ? -1 : dp[^1];
    }
}

