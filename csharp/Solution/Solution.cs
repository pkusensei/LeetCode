using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool CanJump(int[] nums)
    {
        int n = nums.Length;
        int rightmost = 0;
        for (int i = 0; i < n; i++)
        {
            if (i > rightmost) { break; } // unreachable state
            rightmost = int.Max(rightmost, i + nums[i]);
            if (rightmost >= n - 1) { return true; }
        }
        return false;
    }
}