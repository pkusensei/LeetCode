using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] SearchRange(int[] nums, int target)
    {
        int[] res = new int[2];
        Array.Fill(res, -1);
        if (nums.Length == 0) { return res; }
        int left = 0;
        int right = nums.Length - 1;
        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (nums[mid] < target) { left = 1 + mid; }
            else { right = mid; }
        }
        if (nums[left] != target) { return res; }
        res[0] = left;
        right = nums.Length - 1;
        while (left < right)
        {
            int mid = left + (right - left + 1) / 2;
            if (nums[mid] <= target) { left = mid; }
            else { right = mid - 1; }
        }
        res[1] = left;
        return res;
    }
}
