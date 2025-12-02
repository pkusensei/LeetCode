using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountTrapezoids(int[][] points)
    {
        const long M = 1_000_000_007;
        Dictionary<int, long> freq = [];
        foreach (var p in points)
        {
            if (!freq.TryAdd(p[1], 1)) { freq[p[1]] += 1; }
        }
        long res = 0;
        long prefix = 0;
        foreach (var v in freq.Values)
        {
            long curr = v * (v - 1) / 2 % M;
            res = (res + curr * prefix) % M;
            prefix = (prefix + curr) % M;
        }
        return (int)res;
    }
}

