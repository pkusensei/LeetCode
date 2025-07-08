using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Divide(int dividend, int divisor)
    {
        if (divisor == 0 || dividend == int.MinValue && divisor == -1) { return int.MaxValue; }
        int sign = (dividend > 0) ^ (divisor > 0) ? -1 : 1;
        int res = 0;
        long top = Math.Abs((long)dividend);
        long bot = Math.Abs((long)divisor);
        while (top >= bot)
        {
            int shift = 0;
            while (top >= (bot << shift)) { shift += 1; }
            shift -= 1; // Offset last shift to ensure top >= bot
            res += 1 << shift;
            top -= bot << shift;
        }
        return res * sign;
    }
}
