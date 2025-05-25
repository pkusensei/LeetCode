using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LengthAfterTransformations(string s, int t, IList<int> nums)
    {
        const int N = 26;
        const long M = 1_000_000_007;

        Span<int> freq = stackalloc int[N];
        foreach (var ch in s) { freq[ch - 'a'] += 1; }
        long[,] mat = new long[N, N]; // mat[to, from]
        for (int i = 0; i < N; i++)
        {
            for (int delta = 1; delta <= nums[i]; delta++)
            {
                mat[(i + delta) % N, i] += 1;
            }
        }
        mat = MatPow(mat, t);
        long[] res = new long[N];
        for (int i = 0; i < N; i++)
        {
            for (int j = 0; j < N; j++)
            {
                res[i] = (res[i] + mat[i, j] * freq[j]) % M;
            }
        }
        return (int)res.Aggregate(0L, (a, b) => (a + b) % M);

        static long[,] MatPow(long[,] mat, int pow)
        {
            var id = new long[N, N];
            for (int i = 0; i < N; i++) { id[i, i] = 1; }
            if (pow == 0) { return id; }
            if (pow == 1) { return mat; }
            var half = MatPow(mat, pow >> 1);
            var squared = MatMul(half, half);
            if ((pow & 1) == 0) { return squared; }
            else { return MatMul(squared, mat); }
        }

        static long[,] MatMul(long[,] a, long[,] b)
        {
            long[,] res = new long[N, N];
            for (int i = 0; i < N; i++)
            {
                for (int j = 0; j < N; j++)
                {
                    for (int k = 0; k < N; k++)
                    {
                        res[i, j] += a[i, k] * b[k, j];
                        res[i, j] %= M;
                    }
                }
            }
            return res;
        }
    }
}
