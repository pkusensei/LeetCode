using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountTrapezoids(int[][] points)
    {
        const long M = 1_000_000_007;
        Dictionary<int, long> freq = [];
        foreach (var p in points)
        {
            if (!freq.TryAdd(p[1], 1)) { freq[p[1]] += 1; }
        }
        long res = 0;
        long prefix = 0;
        foreach (var v in freq.Values)
        {
            long curr = v * (v - 1) / 2 % M;
            res = (res + curr * prefix) % M;
            prefix = (prefix + curr) % M;
        }
        return (int)res;
    }

    public int CountTrapezoids2(int[][] points)
    {
        Dictionary<(int dx, int dy), long> slopes = [];
        Dictionary<(int dx, int dy, int intercept), long> lines = [];
        Dictionary<(int x_sum, int y_sum), long> midpoints = [];
        Dictionary<(int x_sum, int y_sum, int dx, int dy, int intercept), long> midlines = [];
        for (int i1 = 0; i1 < points.Length; i1++)
        {
            (int x1, int y1) = (points[i1][0], points[i1][1]);
            for (int i2 = 1 + i1; i2 < points.Length; i2++)
            {
                (int x2, int y2) = (points[i2][0], points[i2][1]);
                int dx = x2 - x1;
                int dy = y2 - y1;
                int gcd = GCD(int.Abs(dx), int.Abs(dy));
                dx /= gcd;
                dy /= gcd;
                if (dx < 0 || (dx == 0 && dy < 0))
                {
                    dx *= -1;
                    dy *= -1;
                }
                int intercept = dx * y1 - dy * x1;
                Insert(slopes, (dx, dy));
                Insert(lines, (dx, dy, intercept));
                Insert(midpoints, (x1 + x2, y1 + y2));
                Insert(midlines, (x1 + x2, y1 + y2, dx, dy, intercept));
            }
        }
        return (int)(Sum(slopes.Values) - Sum(lines.Values) - Sum(midpoints.Values) + Sum(midlines.Values));

        static void Insert<T>(Dictionary<T, long> d, T key)
        {
            if (!d.TryAdd(key, 1)) { d[key] += 1; }
        }
        static int GCD(int a, int b) => a == 0 ? b : GCD(b % a, a);
        static long Sum(IEnumerable<long> col) => col.Select(v => v * (v - 1) / 2).Sum();
    }
}

