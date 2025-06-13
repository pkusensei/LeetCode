using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinimizeMax(int[] nums, int p)
    {
        Array.Sort(nums);
        int left = 0;
        int right = 1_000_000_001;
        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (Count(mid) >= p) { right = mid; }
            else { left = 1 + mid; }
        }
        return left;

        int Count(int mid)
        {
            int res = 0;
            for (int i = 0; i < nums.Length - 1; i += 1)
            {
                if (nums[1 + i] - nums[i] <= mid)
                {
                    res += 1;
                    i += 1;
                }
            }
            return res;
        }
    }
}
