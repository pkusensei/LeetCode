using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsRectangleOverlap(int[] rec1, int[] rec2)
    {
        int xmin = int.Max(rec1[0], rec2[0]);
        int xmax = int.Min(rec1[2], rec2[2]);
        int ymin = int.Max(rec1[1], rec2[1]);
        int ymax = int.Min(rec1[3], rec2[3]);
        return xmin < xmax && ymin < ymax;
    }
}
