using System.Numerics;
using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public long CountFairPairs(int[] nums, int lower, int upper)
    {
        Array.Sort(nums);
        return LessThan(nums, 1 + upper) - LessThan(nums, lower);

        static long LessThan(Span<int> nums, int value)
        {
            int left = 0;
            int right = nums.Length - 1;
            long res = 0;
            while (left < right)
            {
                int sum = nums[left] + nums[right];
                if (sum < value)
                {
                    // this pair is valid, so is everything between
                    res += right - left;
                    left += 1;
                }
                else
                {
                    right -= 1; // pair sum too big
                }
            }
            return res;
        }
    }
}
