using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long CountSubarrays(int[] nums, int minK, int maxK)
    {
        int prev_min = -1;
        int prev_max = -1;
        int start = 0;
        long res = 0;
        for (int i = 0; i < nums.Length; i += 1)
        {
            int num = nums[i];
            if (num < minK || maxK < num)
            {
                prev_min = -1;
                prev_max = -1;
                start = 1 + i;
            }
            if (num == minK) { prev_min = i; }
            if (num == maxK) { prev_max = i; }
            if (prev_min > -1 && prev_max > -1)
            {
                res += Math.Min(prev_min, prev_max) - start + 1;
            }
        }
        return res;
    }
}
