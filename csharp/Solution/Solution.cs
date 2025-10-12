using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MagicalSum(int m, int k, int[] nums)
    {
        const long MOD = 1_000_000_007;
        long[,] comb = PrecomputeCombinations(m);
        Dictionary<(int, int, int, int), long> memo = [];
        return (int)Dfs(m, k, 0, 0);

        long Dfs(int m, int k, int idx, int carry)
        {
            if (k < 0 || m + int.PopCount(carry) < k) { return 0; }
            if (m == 0) { return int.PopCount(carry) == k ? 1 : 0; }
            if (idx >= nums.Length) { return 0; }
            var key = (m, k, idx, carry);
            if (memo.TryGetValue(key, out var v)) { return v; }
            long res = 0;
            for (int count = 0; count <= m; count++)
            {
                // nums[idx] appears `count` times
                // and has `comb[m, count]` possible slots
                long curr = comb[m, count] * ModPow(nums[idx], count) % MOD;
                int new_carry = carry + count;
                res += curr * Dfs(m - count, k - (new_carry & 1), 1 + idx, new_carry >> 1);
                res %= MOD;
            }
            memo.Add(key, res);
            return res;
        }

        static long[,] PrecomputeCombinations(int m)
        {
            long[,] res = new long[1 + m, 1 + m];
            res[0, 0] = 1;
            for (int r = 1; r <= m; r++)
            {
                res[r, 0] = 1;
                for (int c = 1; c <= r; c++)
                {
                    res[r, c] = (res[r - 1, c - 1] + res[r - 1, c]) % MOD;
                }
            }
            return res;
        }

        static long ModPow(long b, long exp)
        {
            if (exp == 0) { return 1; }
            return (exp & 1) == 0 ? ModPow(b * b % MOD, exp >> 1)
                : ModPow(b * b % MOD, exp >> 1) * b % MOD;
        }
    }
}
