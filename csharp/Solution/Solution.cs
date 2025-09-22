using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] FindErrorNums(int[] nums)
    {
        int n = nums.Length;
        int dup = 0;
        int sum = 0;
        for (int i = 0; i < n; i++)
        {
            sum += int.Abs(nums[i]);
            int idx = int.Abs(nums[i]) - 1;
            if (nums[idx] < 0) { dup = int.Abs(nums[i]); }
            nums[idx] *= -1;
        }
        return [dup, n * (1 + n) / 2 - sum + dup];
    }
}