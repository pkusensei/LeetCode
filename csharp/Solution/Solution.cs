using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int PartitionDisjoint(int[] nums)
    {
        int n = nums.Length;
        List<int> min_right = new(n);
        foreach (var item in nums.Reverse())
        {
            if (min_right.Count > 1) { min_right.Add(int.Min(item, min_right.Last())); }
            else { min_right.Add(item); }
        }
        min_right.Reverse();
        int pref_max = nums[0];
        for (int i = 1; i < n; i++)
        {
            if (pref_max <= min_right[i]) { return i; }
            pref_max = int.Max(pref_max, nums[i]);
        }
        return n;
    }
}