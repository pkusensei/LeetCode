using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaximumBeauty(int[] nums, int k)
    {
        Array.Sort(nums);
        int left = 0;
        int res = 0;
        for (int right = 0; right < nums.Length; right++)
        {
            while (nums[right] - nums[left] > 2 * k) { left += 1; }
            res = Math.Max(res, right + 1 - left);
        }
        return res;
    }
}
