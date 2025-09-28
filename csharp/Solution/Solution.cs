using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LargestPerimeter(int[] nums)
    {
        Array.Sort(nums, (a, b) => b.CompareTo(a));
        for (int i = 0; i < nums.Length - 3; i++)
        {
            if (nums[i] < nums[1 + i] + nums[2 + i])
            {
                return nums[i..(3 + i)].Sum();
            }
        }
        return 0;
    }
}