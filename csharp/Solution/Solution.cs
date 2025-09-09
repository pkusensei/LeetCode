using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindUnsortedSubarray(int[] nums)
    {
        int min = 100_001;
        int max = -100_001;
        int start = 0;
        int end = -1;
        int left = 0;
        int right = nums.Length - 1;
        // Finds leftmost and rightmost positions where elements are "out of place"
        while (right >= 0)
        {
            if (nums[right] <= min) { min = nums[right]; }
            else { start = right; }
            if (nums[left] >= max) { max = nums[left]; }
            else { end = left; }
            left += 1;
            right -= 1;
        }
        return end + 1 - start;
    }
}
