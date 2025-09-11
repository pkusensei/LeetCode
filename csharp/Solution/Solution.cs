using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool ValidSquare(int[] p1, int[] p2, int[] p3, int[] p4)
    {
        Point[] points = [.. new[] { p1, p2, p3, p4 }.Select(v => new Point(v))];
        HashSet<int> s = [];
        for (int i = 0; i < 4; i++)
        {
            for (int i2 = 1 + i; i2 < 4; i2++)
            {
                s.Add(points[i].Dist(points[i2]));
            }
        }
        return s.Count == 2 && s.All(v => v > 0);
    }
}

readonly record struct Point(int X, int Y)
{
    public Point(int[] v) : this(v[0], v[1]) { }
    public int Dist(Point other)
        => (int)(Math.Pow(X - other.X, 2) + Math.Pow(Y - other.Y, 2));
}