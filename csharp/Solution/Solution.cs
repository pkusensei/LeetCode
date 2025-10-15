using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindShortestSubArray(int[] nums)
    {
        Dictionary<int, int> freq = [];
        Dictionary<int, int> left = [];
        Dictionary<int, int> right = [];
        int max_f = 1;
        for (int i = 0; i < nums.Length; i++)
        {
            if (!freq.TryAdd(nums[i], 1))
            {
                freq[nums[i]] += 1;
                max_f = int.Max(max_f, freq[nums[i]]);
            }
            left.TryAdd(nums[i], i);
            right[nums[i]] = i;
        }
        int res = nums.Length;
        foreach (var (num, f) in freq)
        {
            if (f == max_f)
            {
                res = int.Min(res, right[num] + 1 - left[num]);
            }
        }
        return res;
    }
}
