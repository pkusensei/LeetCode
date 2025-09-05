using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int SingleNonDuplicate(int[] nums)
    {
        int n = nums.Length;
        int left = 0;
        int right = n - 1;
        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if ((mid & 1) == 1)
            {
                if (nums[mid] == nums[mid - 1]) { left = 1 + mid; }
                else { right = mid - 1; }
            }
            else if (nums[mid] == nums[1 + mid]) { left = mid; }
            else { right = mid; }
        }
        return nums[left];
    }
}
