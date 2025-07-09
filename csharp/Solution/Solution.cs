using System.Reflection.Metadata;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FirstMissingPositive(int[] nums)
    {
        int n = nums.Length;
        bool has1 = false;
        for (int i = 0; i < n; i++)
        {
            has1 |= nums[i] == 1;
            if (nums[i] <= 0 || n < nums[i]) { nums[i] = 1; }
        }
        if (!has1) { return 1; }
        for (int i = 0; i < n; i++)
        {
            int val = Math.Abs(nums[i]);
            if (val == n) { nums[0] = -Math.Abs(nums[0]); }
            else { nums[val] = -Math.Abs(nums[val]); }
        }
        for (int i = 1; i < n; i++)
        {
            if (nums[i] > 0) { return i; }
        }
        if (nums[0] > 0) { return n; }
        return 1 + n;
    }

    public int WithCycleSort(int[] nums)
    {
        int n = nums.Length;
        int i = 0;
        while (i < n)
        {
            int correct_idx = nums[i] - 1;
            if (0 < nums[i] && nums[i] <= n && nums[i] != nums[correct_idx])
            {
                (nums[i], nums[correct_idx]) = (nums[correct_idx], nums[i]);
            }
            else { i += 1; }
        }
        for (i = 0; i < n; i++)
        {
            if (nums[i] != 1 + i) { return 1 + i; }
        }
        return 1 + n;
    }
}
