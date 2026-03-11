using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumMusicPlaylists(int n, int goal, int k)
    {
        const long M = 1_000_000_007;
        // dp[x][y] = `x` songs make up playlist of length `y`
        long[,] dp = new long[1 + n, 1 + goal];
        dp[0, 0] = 1;
        for (int song = 1; song <= n; song++)
        {
            for (int len = 1; len <= goal; len++)
            {
                // (song-1) make up (len-1)
                // To append a song, we have (n-(song-1)) to choose from
                dp[song, len] = dp[song - 1, len - 1] * (n - (song - 1)) % M;
                if (song > k)
                {
                    // Pick a used one
                    // The last `k` songs are banned
                    dp[song, len] += dp[song, len - 1] * (song - k) % M;
                }
                dp[song, len] %= M;
            }
        }
        return (int)dp[n, goal];
    }
}