using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumberOfPairs(int[][] points)
    {
        Array.Sort(points,
            (a, b) => a[0] == b[0] ? b[1].CompareTo(a[1]) : a[0].CompareTo(b[0]));
        int res = 0;
        for (int i1 = 0; i1 < points.Length; i1++)
        {
            int y1 = points[i1][1];
            int ymax = int.MinValue;
            for (int i2 = 1 + i1; i2 < points.Length; i2++)
            {
                int y2 = points[i2][1];
                if (y1 >= y2 && y2 > ymax)
                {
                    res += 1;
                    ymax = y2;
                }
            }
        }
        return res;
    }
}