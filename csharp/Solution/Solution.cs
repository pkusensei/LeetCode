using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public double MinAreaFreeRect(int[][] points)
    {
        int n = points.Length;
        Dictionary<(int x, int y, long d), List<(int x, int y)>> dict = [];
        double res = double.MaxValue;
        for (int i1 = 0; i1 < n; i1++)
        {
            (int x1, int y1) = (points[i1][0], points[i1][1]);
            for (int i2 = 1 + i1; i2 < n; i2++)
            {
                (int x2, int y2) = (points[i2][0], points[i2][1]);
                int x = x1 + x2;
                int y = y1 + y2;
                long d = (long)(Math.Pow(x1 - x2, 2) + Math.Pow(y1 - y2, 2));
                if (dict.TryGetValue((x, y, d), out var list))
                {
                    foreach (var p in list)
                    {
                        long a = (long)(Math.Pow(p.x - x1, 2) + Math.Pow(p.y - y1, 2));
                        long b = (long)(Math.Pow(p.x - x2, 2) + Math.Pow(p.y - y2, 2));
                        res = double.Min(res, Math.Sqrt(a * b));
                    }
                    list.Add((x1, y1));
                }
                else
                {
                    dict.Add((x, y, d), [(x1, y1)]);
                }
            }
        }
        return res < double.MaxValue ? res : 0.0;
    }
}
