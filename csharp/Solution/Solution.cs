using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinStickers(string[] stickers, string target)
    {
        int n = target.Length;
        int[] dp = new int[1 << n];
        Array.Fill(dp, -1);
        dp[0] = 0;
        for (int mask = 0; mask < (1 << n); mask++)
        {
            if (dp[mask] < 0) { continue; } // unreachable
            foreach (var s in stickers)
            {
                int curr = mask;
                foreach (var ch in s)
                {
                    for (int i = 0; i < n; i++)
                    {
                        if (((curr >> i) & 1) == 1) { continue; }
                        if (ch == target[i])
                        {
                            curr |= 1 << i;
                            break;
                        }
                    }
                }
                if (dp[curr] == -1 || dp[curr] > 1 + dp[mask])
                {
                    dp[curr] = 1 + dp[mask];
                }
            }
        }
        return dp[^1];
    }
}
