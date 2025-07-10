using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[][] Insert(int[][] intervals, int[] newInterval)
    {
        List<int[]> res = [];
        int start = newInterval[0];
        int end = newInterval[1];
        foreach (var item in intervals)
        {
            if (end < item[0] || item[1] < start)
            {
                res.Add(item);
                continue;
            }
            start = int.Min(start, item[0]);
            end = int.Max(end, item[1]);
        }
        res.Add([start, end]);
        res.Sort((a, b) => a[0].CompareTo(b[0]));
        return [.. res];
    }
}