using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[][] Merge(int[][] intervals)
    {
        Array.Sort(intervals, (a, b) =>
        {
            if (a[0] == b[0]) { return a[1].CompareTo(b[1]); }
            return a[0].CompareTo(b[0]);
        });
        List<int[]> res = [];
        int start = intervals[0][0];
        int end = intervals[0][1];
        foreach (var item in intervals.Skip(1))
        {
            if (end < item[0])
            {
                res.Add([start, end]);
                start = item[0];
            }
            end = int.Max(end, item[1]);
        }
        res.Add([start, end]);
        return [.. res];
    }
}