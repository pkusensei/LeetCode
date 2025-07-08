using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Search(int[] nums, int target)
    {
        if (nums[0] <= nums.Last())
        {
            int val = Array.BinarySearch(nums, target);
            return val >= 0 ? val : -1;
        }
        int left = 0;
        int right = nums.Length - 1;
        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (nums[mid] < nums.Last()) { right = mid - 1; }
            else { left = mid + 1; }
        }
        int pivot = nums[left] > nums.Last() ? left + 1 : left;
        int res;
        if (target >= nums[0]) { res = Array.BinarySearch(nums, 0, pivot, target); }
        else { res = Array.BinarySearch(nums, pivot, nums.Length - pivot, target); }
        return res >= 0 ? res : -1;
    }
}
