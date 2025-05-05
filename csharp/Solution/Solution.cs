using System.Buffers;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumTilings(int n)
    {
        const int M = 1_000_000_007;
        Span<int> dp = stackalloc int[1001];
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 5;
        for (int i = 4; i <= n; i++)
        {
            dp[i] = (2 * dp[i - 1] % M + dp[i - 3]) % M;
        }
        return dp[n];
    }
}
