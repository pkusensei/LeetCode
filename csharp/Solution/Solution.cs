using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class KthLargest
{
    public int Search(int[] nums, int target)
    {
        int left = 0;
        int right = nums.Length - 1;
        while (left <= right)
        {
            int mid = left + (right - left) / 2;
            if (target == nums[mid]) { return mid; }
            else if (target < nums[mid]) { right = mid - 1; }
            else { left = 1 + mid; }
        }
        return -1;
    }
}
