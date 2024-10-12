// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int Search(int[] nums, int target)
    {
        var (left, right) = (0, nums.Length - 1);
        while (left <= right)
        {
            var mid = left + (right - left) / 2;
            if (nums[mid] == target) { return mid; }
            else if (nums[mid] < target) { left = mid + 1; }
            else { right = mid - 1; }
        }
        return -1;
    }
}
