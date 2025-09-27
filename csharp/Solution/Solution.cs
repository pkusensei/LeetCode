using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public double LargestTriangleArea(int[][] points)
    {
        int n = points.Length;
        double res = 0.0;
        Span<double> x = stackalloc double[3];
        Span<double> y = stackalloc double[3];
        for (int i0 = 0; i0 < n; i0++)
        {
            x[0] = points[i0][0];
            y[0] = points[i0][1];
            for (int i1 = 1 + i0; i1 < n; i1++)
            {
                x[1] = points[i1][0];
                y[1] = points[i1][1];
                for (int i2 = 1 + i1; i2 < n; i2++)
                {
                    x[2] = points[i2][0];
                    y[2] = points[i2][1];
                    double curr = 0.0;
                    for (int p = 0; p < 3; p++)
                    {
                        curr += x[p] * y[(1 + p) % 3] - x[(1 + p) % 3] * y[p];
                    }
                    curr = 0.5 * double.Abs(curr);
                    res = double.Max(res, curr);
                }
            }

        }
        return res;
    }
}