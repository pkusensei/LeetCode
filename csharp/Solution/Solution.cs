using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int WiggleMaxLength(int[] nums)
    {
        int dec = 0;
        int inc = 0;
        for (int i = 0; i < nums.Length - 1; i++)
        {
            if (nums[i] < nums[1 + i]) { inc = 1 + dec; }
            if (nums[i] > nums[1 + i]) { dec = 1 + inc; }
        }
        return 1 + int.Max(inc, dec);
    }
}