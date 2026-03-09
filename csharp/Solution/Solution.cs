using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxSubarraySumCircular(int[] nums)
    {
        int min_sum = int.MaxValue;
        int max_sum = int.MinValue;
        int curr_min = 0;
        int curr_max = 0;
        int sum = 0;
        foreach (var item in nums)
        {
            curr_min = int.Min(curr_min + item, item);
            curr_max = int.Max(curr_max + item, item);
            min_sum = int.Min(curr_min, min_sum);
            max_sum = int.Max(max_sum, curr_max);
            sum += item;
        }
        return max_sum < 0 ? max_sum : int.Max(max_sum, sum - min_sum);
    }
}