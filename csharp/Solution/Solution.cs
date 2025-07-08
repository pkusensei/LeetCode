using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public void NextPermutation(int[] nums)
    {
        int left = -1;
        for (int i = nums.Length - 2; i >= 0; i -= 1)
        {
            if (nums[i] < nums[1 + i])
            {
                left = i;
                break;
            }
        }
        if (left == -1)
        {
            Array.Sort(nums);
            return;
        }
        int right = -1;
        for (int i = nums.Length - 1; i > left; i -= 1)
        {
            if (nums[left] < nums[i])
            {
                right = i;
                break;
            }
        }
        if (right != -1) { (nums[left], nums[right]) = (nums[right], nums[left]); }
        Array.Reverse(nums, 1 + left, nums.Length - left - 1);
    }
}
