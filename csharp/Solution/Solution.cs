using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int KInversePairs(int n, int k)
    {
        const int M = 1_000_000_007;
        if (k == 0) { return 1; }
        int[] dp = new int[1 + k];
        dp[0] = 1;
        for (int ni = 1; ni <= n; ni++)
        {
            int[] curr = new int[1 + k];
            // sum of sliding window of size `ni`
            int sum = 0;
            for (int ki = 0; ki <= k; ki++)
            {
                sum += dp[ki];
                if (ki >= ni) { sum -= dp[ki - ni]; }
                if (sum < 0) { sum += M; }
                sum %= M;
                curr[ki] = sum;
            }
            dp = curr;
        }
        return dp[k];
    }
}