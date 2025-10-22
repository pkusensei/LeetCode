using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int SmallestDistancePair(int[] nums, int k)
    {
        Array.Sort(nums);
        int left = 0;
        int right = nums[^1] - nums[0];
        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (Count(mid) >= k) { right = mid; }
            else { left = 1 + mid; }
        }
        return left;

        int Count(int mid)
        {
            int left = 0;
            int count = 0;
            for (int right = 0; right < nums.Length; right += 1)
            {
                while (nums[right] - nums[left] > mid)
                {
                    left += 1;
                }
                count += right - left;
            }
            return count;
        }
    }
}
