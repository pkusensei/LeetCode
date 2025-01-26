using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int NumWays(string[] words, string target)
    {
        var wn = words[0].Length;
        var tn = target.Length;
        int[,] freq = new int[wn, 26];
        for (int i = 0; i < wn; i++)
        {
            foreach (var item in words)
            {
                freq[i, item[i] - 'a'] += 1;
            }
        }
        var memo = new long[wn, tn];
        for (int i = 0; i < wn; i++)
        {
            for (int j = 0; j < tn; j++)
            {
                memo[i, j] = -1;
            }
        }
        return (int)Dfs(0, 0);

        long Dfs(int fi, int ti)
        {
            long mod = 1_000_000_007;
            if (ti >= tn) { return 1; }
            if (wn - fi < tn - ti) { return 0; }
            if (memo[fi, ti] > -1) { return memo[fi, ti]; }
            var res = Dfs(1 + fi, ti);
            res += freq[fi, target[ti] - 'a'] * Dfs(1 + fi, 1 + ti);
            res %= mod;
            memo[fi, ti] = res;
            return res;
        }
    }
}
