using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int EraseOverlapIntervals(int[][] intervals)
    {
        Array.Sort(intervals, (a, b) =>
        {
            if (a[1] == b[1]) { return a[0].CompareTo(b[0]); }
            return a[1].CompareTo(b[1]);
        });
        int prev = int.MinValue;
        int res = 0;
        foreach (var item in intervals)
        {
            if (prev > item[0]) { res += 1; }
            else { prev = int.Max(prev, item[1]); }
        }
        return res;
    }
}