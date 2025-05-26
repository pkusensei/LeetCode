using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountBalancedPermutations(string num)
    {
        Span<int> freq = stackalloc int[10];
        int sum = 0;
        foreach (var c in num)
        {
            freq[c - '0'] += 1;
            sum += c - '0';
        }
        if ((sum & 1) == 1) { return 0; }
        int n = num.Length;
        long[,,] memo = new long[n, 1 + n / 2, 1 + sum / 2];
        for (int a = 0; a < n; a++)
        {
            for (int b = 0; b < 1 + n / 2; b++)
            {
                for (int c = 0; c < 1 + sum / 2; c++)
                {
                    memo[a, b, c] = -1; // set to -1 is necessary to avoid tle
                }
            }
        }
        long res = Dfs(0, n / 2, sum / 2) * Fact[n / 2] % MOD * Fact[n - n / 2] % MOD;
        foreach (var f in freq)
        {
            res = res * InvF[f] % MOD;
        }
        return (int)res;

        long Dfs(int idx, int count, int sum)
        {
            if (count == 0) { return sum == 0 ? 1 : 0; }
            if (idx >= n || sum < 0) { return 0; }
            if (memo[idx, count, sum] > -1) { return memo[idx, count, sum]; }
            long res = (Dfs(1 + idx, count, sum) + Dfs(1 + idx, count - 1, sum - (num[idx] - '0'))) % MOD;
            memo[idx, count, sum] = res;
            return res;
        }
    }

    const int N = 81;
    const long MOD = 1_000_000_007;
    static long[] _fact;
    static long[] _invf;
    static long[] Fact
    {
        get
        {
            if (_fact is null)
            {
                _fact = new long[N];
                _fact[0] = 1;
                for (int i = 1; i < N; i++)
                {
                    _fact[i] = i * _fact[i - 1] % MOD;
                }
            }
            return _fact;
        }
    }
    static long[] InvF
    {
        get
        {
            if (_invf is null)
            {
                _invf = new long[N];
                _invf[N - 1] = ModPow(Fact[N - 1], MOD - 2, MOD);
                for (int i = N - 2; i >= 0; i -= 1)
                {
                    _invf[i] = (1 + i) * _invf[1 + i] % MOD;
                }
            }
            return _invf;
        }
    }

    static long ModPow(long b, long exp, long m)
    {
        if (exp == 0) { return 1; }
        if ((exp & 1) == 0) { return ModPow(b * b % m, exp >> 1, m); }
        return ModPow(b * b % m, exp >> 1, m) * b % m;
    }
}
