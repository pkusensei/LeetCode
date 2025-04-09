using System.Numerics;
using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaximumScore(IList<int> nums, int k)
    {
        var sieve = Sieve(nums.Max());
        var p_scores = nums.Select(val =>
        {
            int score = 0;
            foreach (var p in sieve)
            {
                if (val % p == 0)
                {
                    score += 1;
                    while (val % p == 0) { val /= p; }
                }
            }
            return score;
        }).ToList();
        int[] next_greater = new int[nums.Count];
        Array.Fill(next_greater, nums.Count);
        Stack<int> st = [];
        for (int i = 0; i < p_scores.Count; i++)
        {
            int s = p_scores[i];
            while (st.TryPeek(out var top) && p_scores[top] < s)
            {
                st.Pop();
                next_greater[top] = i;
            }
            st.Push(i);
        }
        st.Clear();
        int[] prev_greater = new int[nums.Count];
        Array.Fill(prev_greater, -1);
        for (int i = nums.Count - 1; i >= 0; i -= 1)
        {
            int s = p_scores[i];
            while (st.TryPeek(out var top) && p_scores[top] <= s)
            {
                st.Pop();
                prev_greater[top] = i;
            }
            st.Push(i);
        }
        long k_ = k;
        long res = 1;
        long mod = 1_000_000_007;
        foreach (var (val, i) in nums.Select((val, i) => (val, i)).OrderByDescending(p => p.val))
        {
            if (k_ <= 0) { break; }
            long count = (next_greater[i] - i) * (i - prev_greater[i]);
            count = Math.Min(count, k_);
            k_ -= count;
            res *= ModPow(val, count, mod);
            res %= mod;
        }
        return (int)res;

        static List<int> Sieve(int num)
        {
            if (num < 2) { return []; }
            var sieve = new bool[1 + num];
            Array.Fill(sieve, true, 2, num - 1);
            for (int p = 2; p <= num; p += 1)
            {
                if (sieve[p])
                {
                    for (int v = 2 * p; v <= num; v += p) { sieve[v] = false; }
                }
            }
            return [.. Enumerable.Range(0, 1 + num).Where(i => sieve[i])];
        }

        static long ModPow(long b, long exp, long mod)
        {
            long res = 1;
            b %= mod;
            while (exp > 0)
            {
                if ((exp & 1) == 1) { res = res * b % mod; }
                exp /= 2;
                b = b * b % mod;
            }
            return res;
        }
    }
}
