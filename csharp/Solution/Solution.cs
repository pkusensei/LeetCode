using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaximumGap(int[] nums)
    {
        int n = nums.Length;
        int min_num = nums.Min();
        int max_num = nums.Max();
        if (n < 2 || min_num == max_num) { return max_num - min_num; }
        int bucket_gap = int.Max(1, (max_num - min_num) / (n - 1));
        int bucket_count = 1 + (max_num - min_num) / bucket_gap;
        (int min, int max)[] buckets = new (int, int)[bucket_count];
        Array.Fill(buckets, (int.MaxValue, int.MinValue));
        foreach (var num in nums)
        {
            int idx = (num - min_num) / bucket_gap;
            buckets[idx].min = int.Min(buckets[idx].min, num);
            buckets[idx].max = int.Max(buckets[idx].max, num);
        }
        int res = 0;
        int prev_max = min_num;
        foreach (var (buc_min, buc_max) in buckets)
        {
            if (buc_min == int.MaxValue) { continue; }
            res = int.Max(res, buc_min - prev_max);
            prev_max = buc_max;
        }
        return res;
    }
}
