using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    // 1 2 3
    // 2 3
    public long PutMarbles(int[] weights, int k)
    {
        if (weights.Length == k || k < 2) { return 0; }
        var nums = weights.Zip(weights.Skip(1))
                          .Select(p => (long)(p.First + p.Second))
                          .Order()
                          .ToList();
        long min = nums[..(k - 1)].Sum();
        long max = nums.TakeLast(k - 1).Sum();
        return max - min;
    }
}
