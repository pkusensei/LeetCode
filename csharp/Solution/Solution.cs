using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaximumUniqueSubarray(int[] nums)
    {
        List<int> prefix = [];
        Dictionary<int, int> seen = [];
        int sum = 0;
        int left = -1;
        int res = 0;
        for (int right = 0; right < nums.Length; right++)
        {
            sum += nums[right];
            prefix.Add(sum);
            if (seen.TryGetValue(nums[right], out var prev))
            {
                left = int.Max(left, prev);
            }
            int curr = sum;
            if (left >= 0) { curr -= prefix[left]; }
            res = int.Max(res, curr);
            seen[nums[right]] = right;
        }
        return res;
    }
}
