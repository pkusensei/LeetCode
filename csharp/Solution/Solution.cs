using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsRectangleCover(int[][] rectangles)
    {
        long total = 0;
        int xmin = int.MaxValue;
        int ymin = int.MaxValue;
        int xmax = int.MinValue;
        int ymax = int.MinValue;
        Dictionary<(int x, int y), int> freq = [];
        foreach (var rec in rectangles)
        {
            int x1 = rec[0];
            int y1 = rec[1];
            int x2 = rec[2];
            int y2 = rec[3];
            xmin = int.Min(xmin, x1);
            ymin = int.Min(ymin, y1);
            xmax = int.Max(xmax, x2);
            ymax = int.Max(ymax, y2);
            total += (x2 - x1) * (long)(y2 - y1);
            foreach (var item in new[] { (x1, y1), (x2, y2), (x1, y2), (x2, y1) })
            {
                if (!freq.TryAdd(item, 1)) { freq[item] += 1; }
            }
        }
        if ((long)(xmax - xmin) * (ymax - ymin) != total) { return false; }
        Span<(int, int)> corners = [(xmin, ymin), (xmax, ymax), (xmin, ymax), (xmax, ymin)];
        foreach (var (k, v) in freq)
        {
            if (corners.Contains(k))
            {
                if (v == 1) { continue; }
            }
            else if ((v & 1) == 0) { continue; }
            return false;
        }
        return true;
    }
}
