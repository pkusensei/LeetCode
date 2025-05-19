using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string TriangleType(int[] nums)
    {
        Array.Sort(nums);
        if (nums[0] + nums[1] <= nums[2]) { return "none"; }
        return nums.Distinct().Count() switch
        {
            1 => "equilateral",
            2 => "isosceles",
            _ => "scalene",
        };
    }
}
