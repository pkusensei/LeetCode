using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int IdealArrays(int n, int maxValue)
    {
        const long M = 1_000_000_007;
        var freq = Enumerable.Range(1, maxValue).Select(v => (v, 1L)).ToDictionary();
        long res = 0;
        long comb = 1;
        for (int i = 0; i < n; i++)
        {
            Dictionary<int, long> curr = [];
            foreach (var (k, count) in freq)
            {
                for (int key = 2 * k; key <= maxValue; key += k)
                {
                    if (!curr.TryAdd(key, count)) { curr[key] += count; }
                }
            }
            res = (res + comb * freq.Values.Sum()) % M;
            comb = comb * (n - i - 1) % M * ModPow(1 + i, M - 2, M) % M;
            freq = curr;
        }
        return (int)res;
    }
}
