using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsMatch(string s, string p)
    {
        int n1 = s.Length;
        int n2 = p.Length;
        byte[,] memo = new byte[1 + n1, 1 + n2];
        return Dfs(s, p);

        bool Dfs(ReadOnlySpan<char> hay, ReadOnlySpan<char> needle)
        {
            int n1 = hay.Length;
            int n2 = needle.Length;
            if (memo[n1, n2] > 0) { return memo[n1, n2] > 1; }
            if (n1 == 0)
            {
                for (int i = 0; i < needle.Length; i++)
                {
                    if (needle[i] != '*')
                    {
                        memo[n1, n2] = 1;
                        return false;
                    }
                }
                memo[n1, n2] = 2;
                return true;
            }
            if (n2 == 0)
            {
                memo[n1, n2] = 1;
                return false;
            }
            if (hay[0] == needle[0] || needle[0] == '?') { return Dfs(hay[1..], needle[1..]); }
            if (needle[0] == '*')
            {
                for (int i = 0; i <= hay.Length; i++)
                {
                    if (Dfs(hay[i..], needle[1..]))
                    {
                        memo[n1, n2] = 2;
                        return true;
                    }
                }
            }
            memo[n1, n2] = 1;
            return false;
        }
    }
}
