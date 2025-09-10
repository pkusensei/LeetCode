using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[][] OuterTrees(int[][] points)
    {
        int n = points.Length;
        if (n <= 1) { return points; }
        Point[] arr = [.. points.Select(v => new Point(v[0], v[1]))];
        Array.Sort(arr, (a, b) => a.X == b.X ? a.Y.CompareTo(b.Y) : a.X.CompareTo(b.X));
        List<Point> lower = [];
        foreach (var p in arr)
        {
            while (lower.Count >= 2 && Point.Cross(lower[^2], lower[^1], p) < 0)
            {
                lower.RemoveAt(lower.Count - 1);
            }
            lower.Add(p);
        }
        List<Point> upper = [];
        foreach (var p in arr.Reverse())
        {
            while (upper.Count >= 2 && Point.Cross(upper[^2], upper[^1], p) < 0)
            {
                upper.RemoveAt(upper.Count - 1);
            }
            upper.Add(p);
        }
        return [.. lower.SkipLast(1).Union(upper.SkipLast(1))
                    .Select(p => new[] { p.X, p.Y })];
    }
}

readonly record struct Point(int X, int Y) : IComparable<Point>
{
    // Find the orientation of two vectors, i.e rotate OA to OB.
    // >0 counter-clockwise,
    // <0 clockwise,
    // =0  collinear
    public static int Cross(Point ori, Point a, Point b)
        => (a.X - ori.X) * (b.Y - ori.Y) - (a.Y - ori.Y) * (b.X - ori.X);

    public static Point[] ConvexHull(Point[] points)
    {
        int n = points.Length;
        if (n <= 1) { return points; }
        Array.Sort(points);
        List<Point> lower = [];
        foreach (var p in points)
        {
            // We want counter-clockwise turns.
            // If the last three points make a clockwise turn (cross product < 0),
            // remove the middle point.
            while (lower.Count >= 2 && Cross(lower[^2], lower[^1], p) < 0)
            {
                lower.RemoveAt(lower.Count - 1);
            }
            lower.Add(p);
        }
        List<Point> upper = [];
        foreach (var p in points.Reverse())
        {
            while (upper.Count >= 2 && Cross(upper[^2], upper[^1], p) < 0)
            {
                upper.RemoveAt(upper.Count - 1);
            }
            upper.Add(p);
        }
        return [.. lower.SkipLast(1).Union(upper.SkipLast(1))];
    }

    public int CompareTo(Point other)
        => X == other.X ? Y.CompareTo(other.Y) : X.CompareTo(other.X);
}
