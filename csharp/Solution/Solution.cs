using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Rob(int[] nums)
    {
        if (nums.Length <= 2) { return nums.Max(); }
        return int.Max(Solve(nums.AsSpan()[..^1]), Solve(nums.AsSpan()[1..]));

        static int Solve(ReadOnlySpan<int> nums)
        {
            int dp0 = nums[0];
            int dp1 = nums[1];
            foreach (var num in nums[2..])
            {
                int curr = dp0 + num;
                dp0 = int.Max(dp0, dp1);
                dp1 = curr;
            }
            return int.Max(dp0, dp1);
        }
    }
}
