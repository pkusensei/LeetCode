using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountGoodArrays(int n, int m, int k)
    {
        const long MOD = 1_000_000_007;
        // From (n-1), find (n-k-1) indices where [i]!=[i-1]
        var c = NChooseK(n - 1, n - k - 1);
        // Fix first number (as one in 1..=m)
        // these (n-k-1) indices have (m-1)^(n-k-1) choices
        var pow = ModPow(m - 1, n - k - 1);
        return (int)(c * pow % MOD * m % MOD);

        static long NChooseK(long n, long k)
        {
            if (k > n) { return 0; }
            k = Math.Min(k, n - k);
            long numerator = 1;
            for (long i = n + 1 - k; i <= n; i++)
            {
                numerator = numerator * i % MOD;
            }
            long denominator = 1;
            for (long i = 1; i <= k; i++)
            {
                denominator = denominator * i % MOD;
            }
            return numerator * ModPow(denominator, MOD - 2) % MOD;
        }

        static long ModPow(long b, long exp)
        {
            if (exp == 0) { return 1; }
            if ((exp & 1) == 0) { return ModPow(b * b % MOD, exp >> 1); }
            else { return ModPow(b * b % MOD, exp >> 1) * b % MOD; }
        }
    }
}
