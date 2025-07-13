using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsScramble(string s1, string s2)
    {
        int n = s1.Length;
        if (n != s2.Length) { return false; }
        byte[,,] memo = new byte[n, n, 1 + n];
        return Dfs(0, 0, n);

        bool Dfs(int i1, int i2, int len)
        {
            if (memo[i1, i2, len] > 0) { return memo[i1, i2, len] > 1; }
            bool flag = true;
            for (int i = 0; i < len; i++)
            {
                if (s1[i1 + i] != s2[i2 + i])
                {
                    flag = false;
                    break;
                }
            }
            if (flag)
            {
                memo[i1, i2, len] = 2;
                return true;
            }
            for (int delta = 1; delta < len; delta++)
            {
                int len1 = delta;
                int len2 = len - delta;
                if (Dfs(i1, i2, len1) && Dfs(i1 + delta, i2 + delta, len2))
                {
                    memo[i1, i2, len] = 2;
                    return true;
                }
                if (Dfs(i1, i2 + len2, len1) && Dfs(i1 + len1, i2, len2))
                {
                    memo[i1, i2, len] = 2;
                    return true;
                }
            }
            memo[i1, i2, len] = 1;
            return false;
        }
    }
}