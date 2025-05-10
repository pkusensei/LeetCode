using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long MinSum(int[] nums1, int[] nums2)
    {
        (var sum1, var zero1) = Count(nums1);
        (var sum2, var zero2) = Count(nums2);
        if (zero1 < zero2)
        {
            (sum1, sum2) = (sum2, sum1);
            (zero1, zero2) = (zero2, zero1);
        }
        if (zero1 == 0) { return sum1 == sum2 ? sum1 : -1; }
        else if (zero2 == 0) { return sum1 + zero1 <= sum2 ? sum2 : -1; }
        else { return Math.Max(sum1 + zero1, sum2 + zero2); }

        static (long sum, int zeros) Count(int[] nums)
        {
            long sum = 0;
            int zeros = 0;
            foreach (var item in nums)
            {
                sum += item;
                if (item == 0) { zeros += 1; }
            }
            return (sum, zeros);
        }
    }
}
