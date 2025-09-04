using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LongestPalindromeSubseq(string s)
    {
        int n = s.Length;
        int[,] dp = new int[n, n];
        for (int left = n - 1; left >= 0; left -= 1)
        {
            dp[left, left] = 1;
            for (int right = 1 + left; right < n; right++)
            {
                if (s[left] == s[right])
                {
                    dp[left, right] = 2 + dp[1 + left, right - 1];
                }
                else
                {
                    dp[left, right] = int.Max(dp[1 + left, right], dp[left, right - 1]);
                }
            }
        }
        return dp[0, n - 1];
        int[,] memo = new int[n, n];
        for (int i1 = 0; i1 < n; i1++)
        {
            for (int i2 = 0; i2 < n; i2++)
            {
                memo[i1, i2] = -1;
            }
        }
        // return Dfs(0, n - 1);

        int Dfs(int left, int right)
        {
            if (left > right) { return 0; }
            if (left == right) { return 1; }
            if (memo[left, right] > -1) { return memo[left, right]; }
            int res;
            if (s[left] == s[right])
            {
                res = 2 + Dfs(1 + left, right - 1);
            }
            else
            {
                res = int.Max(Dfs(left + 1, right), Dfs(left, right - 1));
            }
            memo[left, right] = res;
            return res;
        }
    }
}