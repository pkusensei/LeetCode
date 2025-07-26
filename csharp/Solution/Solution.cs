using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long MaxSubarrays(int n, int[][] conflictingPairs)
    {
        List<int>[] cps = [.. Enumerable.Range(0, 1 + n).Select(_ => new List<int>())];
        foreach (var cp in conflictingPairs)
        {
            int left = int.Min(cp[0], cp[1]);
            int right = int.Max(cp[0], cp[1]);
            cps[right].Add(left);
        }
        int rightmost_left1 = 0;
        int rightmost_left2 = 0;
        long res = 0;
        long[] candids = new long[1 + n];
        // For each right point in array
        // Here we try to count all valid subarrs ending on `right`
        // Only subarrays ends in [1+rightmotst_left1..=right] are valid
        // Those are accumulated in `res`
        // In addition, removing rightmost_left1 (as removing conflict pair)
        // regains (r_left1 - r_left2) subarrs
        // e.g
        // 1  2  3  4, cps: [1,3], [2,3]
        // l2 l1
        // We first find all subarrs end on [3]
        // Later for [4], the same increase/gain on l1=2 is counted again
        for (int right = 1; right <= n; right++)
        {
            foreach (var item in cps[right])
            {
                if (item > rightmost_left1)
                {
                    rightmost_left2 = rightmost_left1;
                    rightmost_left1 = item;
                }
                else if (item > rightmost_left2)
                {
                    rightmost_left2 = item;
                }
            }
            res += right - rightmost_left1;
            candids[rightmost_left1] += rightmost_left1 - rightmost_left2;
        }
        return res + candids.Max();
    }
}