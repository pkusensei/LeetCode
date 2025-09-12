using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaximumProduct(int[] nums)
    {
        Array.Sort(nums);
        return int.Max(nums[^3] * nums[^2] * nums[^1], nums[0] * nums[1] * nums[^1]);
    }
}