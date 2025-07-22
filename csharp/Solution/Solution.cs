using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int ComputeArea(int ax1, int ay1, int ax2, int ay2, int bx1, int by1, int bx2, int by2)
    {
        int cx1 = int.Max(ax1, bx1);
        int cx2 = int.Min(ax2, bx2);
        int cy1 = int.Max(ay1, by1);
        int cy2 = int.Min(ay2, by2);
        return (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1)
            - int.Max(cx2 - cx1, 0) * int.Max(cy2 - cy1, 0);
    }
}