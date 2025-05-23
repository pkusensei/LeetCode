using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long MaximumValueSum(int[] nums, int k, int[][] edges)
    {
        long sum = 0;
        long min_inc = long.MaxValue;
        long min_dec = long.MaxValue;
        int count = 0;
        foreach (var item in nums)
        {
            int xor_val = item ^ k;
            if (xor_val > item)
            {
                sum += xor_val;
                count += 1;
                min_inc = Math.Min(min_inc, xor_val - item);
            }
            else
            {
                sum += item;
                min_dec = Math.Min(min_dec, item - xor_val);
            }
        }
        if ((count & 1) == 0) { return sum; }
        return sum - Math.Min(min_dec, min_inc);
    }
}
