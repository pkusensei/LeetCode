using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LongestSubarray(int[] nums)
    {
        int max = nums.Max();
        int res = 1;
        for (int left = 0; left < nums.Length; left++)
        {
            if (nums[left] < max) { continue; }
            int right = left;
            for (; right < nums.Length && nums[right] == max; right++)
            {
                res = int.Max(res, right - left + 1);
            }
            left = right;
        }
        return res;
    }
}
