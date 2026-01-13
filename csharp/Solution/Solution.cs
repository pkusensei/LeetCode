using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public double SeparateSquares(int[][] squares)
    {
        const double E = 1e-5;
        double left = double.MaxValue;
        double right = double.MinValue;
        double area = 0.0;
        foreach (var sq in squares)
        {
            left = double.Min(left, sq[1]);
            right = double.Max(right, sq[1] + sq[2]);
            area += double.Pow(sq[2], 2);
        }
        while (right - left > E)
        {
            double mid = (left + right) / 2;
            if (Check(mid)) { left = mid; }
            else { right = mid; }
        }
        return left;

        bool Check(double mid)
        {
            double bot = 0;
            foreach (var sq in squares)
            {
                if (mid >= sq[1] + sq[2]) { bot += double.Pow(sq[2], 2); }
                else if (mid >= sq[1]) { bot += (mid - sq[1]) * sq[2]; }
            }
            return area > 2 * bot;
        }
    }
}
