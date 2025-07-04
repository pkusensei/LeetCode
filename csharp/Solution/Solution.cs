using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsMatch(string s, string p)
    {
        return Dfs(s, p);

        static bool Dfs(ReadOnlySpan<char> hay, ReadOnlySpan<char> needle)
        {
            if (needle.Length == 0) { return hay.Length == 0; }
            bool first = hay.Length > 0 && (hay[0] == needle[0] || needle[0] == '.');
            if (needle.Length > 1 && needle[1] == '*')
            {   // zero or more
                return Dfs(hay, needle[2..]) || (first && Dfs(hay[1..], needle));
            }
            else
            {
                return first && Dfs(hay[1..], needle[1..]);
            }
        }
    }

    public bool WithDp(string s, string p)
    {
        int n1 = s.Length;
        int n2 = p.Length;
        bool[,] dp = new bool[1 + n1, 1 + n2];
        dp[n1, n2] = true;
        for (int i1 = n1; i1 >= 0; i1 -= 1)
        {
            for (int i2 = n2 - 1; i2 >= 0; i2 -= 1)
            {
                bool first = i1 < n1 && (s[i1] == p[i2] || p[i2] == '.');
                if (1 + i2 < n2 && p[1 + i2] == '*')
                {
                    dp[i1, i2] = dp[i1, 2 + i2] || (first && dp[1 + i1, i2]);
                }
                else
                {
                    dp[i1, i2] = first && dp[1 + i1, 1 + i2];
                }
            }
        }
        return dp[0, 0];
    }
}
