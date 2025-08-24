using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LongestSubarray(int[] nums)
    {
        int res = 0;
        int left = 0;
        int zeros = 0;
        for (int right = 0; right < nums.Length; right++)
        {
            if (nums[right] == 0) { zeros += 1; }
            while (zeros > 1)
            {
                zeros -= nums[left] == 0 ? 1 : 0;
                left += 1;
            }
            res = int.Max(res, right + 1 - left);
        }
        return res - 1;
    }
}