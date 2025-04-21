using System.Numerics;
using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    long[,] memo;

    public long NumberOfPowerfulInt(long start, long finish, int limit, string s)
    {
        return LessThan(finish, limit, s) - LessThan(start - 1, limit, s);
    }

    long LessThan(long num, int limit, ReadOnlySpan<char> s)
    {
        var numstr = num.ToString();
        memo = new long[numstr.Length, 2];
        for (int i = 0; i < numstr.Length; i++)
        {
            for (int j = 0; j < 2; j++)
            {
                memo[i, j] = -1;
            }
        }
        return Dfs(numstr, s, limit, 0, 1);
    }

    long Dfs(ReadOnlySpan<char> num, ReadOnlySpan<char> s, int limit, int idx, int tight)
    {
        if (num.Length < s.Length) { return 0; }
        if (idx >= num.Length) { return 1; }
        if (memo[idx, tight] > -1) { return memo[idx, tight]; }
        int upper = limit;
        int numdigit = num[idx] - '0';
        if (tight > 0) { upper = Math.Min(upper, numdigit); }
        long res = 0;
        int pre_len = num.Length - s.Length;
        if (idx >= pre_len)
        {
            int sdigit = s[idx - pre_len] - '0';
            if (upper > sdigit) { return 1; }
            else if (upper < sdigit) { return 0; }
            else { res += Dfs(num, s, limit, 1 + idx, tight & 1); }
        }
        else
        {
            for (int d = 0; d <= upper; d++)
            {
                int next_tight = tight & (numdigit == d ? 1 : 0);
                res += Dfs(num, s, limit, 1 + idx, next_tight);
            }
        }
        memo[idx, tight] = res;
        return res;
    }
}
