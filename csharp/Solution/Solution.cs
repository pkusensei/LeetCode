using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumFactoredBinaryTrees(int[] arr)
    {
        const long M = 1_000_000_007;
        Array.Sort(arr);
        Dictionary<int, long> prev = [];
        foreach (var num in arr)
        {
            Dictionary<int, long> curr = new(prev) { [num] = 1 };
            foreach (var (k, v) in prev)
            {
                var x = num / k;
                if (x * k == num)
                {
                    curr[num] = (curr[num] + v * prev.GetValueOrDefault(x)) % M;
                }
            }
            prev = curr;
        }
        return (int)prev.Values.Aggregate(0L, (a, b) => (a + b) % M);
    }
}
