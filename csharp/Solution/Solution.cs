using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int IntersectionSizeTwo(int[][] intervals)
    {
        Array.Sort(intervals, (a, b) =>
        {
            if (a[1] == b[1]) { return b[0].CompareTo(a[0]); }
            return a[1].CompareTo(b[1]);
        });
        int res = 2;
        int right = intervals[0][1];
        int left = right - 1;
        foreach (var item in intervals[1..])
        {
            int a = item[0];
            int b = item[1];
            if (a <= left) { }
            else if (a <= right)
            {
                left = right;
                right = b;
                res += 1;
            }
            else
            {
                right = b;
                left = right - 1;
                res += 2;
            }
        }
        return res;
    }
}

