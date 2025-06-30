using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindLHS(int[] nums)
    {
        Array.Sort(nums);
        int res = 0;
        int left = 0;
        for (int right = 0; right < nums.Length; right++)
        {
            while (nums[right] - nums[left] > 1) { left += 1; }
            if (nums[right] - nums[left] == 1) { res = Math.Max(res, right + 1 - left); }
        }
        return res;
    }
}
