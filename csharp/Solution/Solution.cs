using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindMaxLength(int[] nums)
    {
        int prefix = 0;
        int res = 0;
        Dictionary<int, int> seen = new() { [0] = -1 };
        for (int i = 0; i < nums.Length; i++)
        {
            prefix += nums[i] > 0 ? 1 : -1;
            if (seen.TryGetValue(prefix, out var prev)) { res = int.Max(res, i - prev); }
            seen.TryAdd(prefix, i);
        }
        return res;
    }
}