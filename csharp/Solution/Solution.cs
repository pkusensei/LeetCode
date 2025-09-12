using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int TriangleNumber(int[] nums)
    {
        int n = nums.Length;
        Array.Sort(nums);
        int res = 0;
        for (int i = n - 1; i >= 2; i -= 1)
        {
            int left = 0;
            int right = i - 1;
            while (left < right)
            {
                if (nums[left] + nums[right] > nums[i])
                {
                    res += right - left;
                    right -= 1;
                }
                else
                {
                    left += 1;
                }
            }
        }
        return res;
    }
}