using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxSum(int[] nums)
    {
        int max = nums.Max();
        if (max <= 0) { return max; }
        return nums.Where(v => v > 0).Distinct().Sum();
    }
}