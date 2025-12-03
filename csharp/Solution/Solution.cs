using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumSubarrayBoundedMax(int[] nums, int left, int right)
    {
        int res = 0;
        int curr = 0;
        int slow = 0;
        for (int fast = 0; fast < nums.Length; fast++)
        {
            if (left <= nums[fast] && nums[fast] <= right) { curr = fast - slow + 1; }
            else if (right < nums[fast])
            {
                slow = 1 + fast;
                curr = 0;
            }
            res += curr;
        }
        return res;
    }
}

