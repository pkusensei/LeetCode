using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool CheckSubarraySum(int[] nums, int k)
    {
        Dictionary<int, int> seen = new() { [0] = -1 };
        int prefix = 0;
        for (int i = 0; i < nums.Length; i++)
        {
            prefix = (prefix + nums[i]) % k;
            if (seen.TryGetValue(prefix, out var prev) && i - prev >= 2)
            {
                return true;
            }
            seen.TryAdd(prefix, i);
        }
        return false;
    }
}