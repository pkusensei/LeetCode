using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int TupleSameProduct(int[] nums)
    {
        Dictionary<int, int> counts = [];
        foreach (var (i, a) in nums.Select((v, i) => (i, v)))
        {
            foreach (var b in nums.Skip(1 + i))
            {
                if (!counts.TryAdd(a * b, 1)) { counts[a * b] += 1; }
            }
        }
        return counts.Values.Where(v => v > 1).Select(v => v * (v - 1) * 4).Sum();
    }
}
