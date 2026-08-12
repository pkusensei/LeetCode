using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NthUglyNumber(int n, int a, int b, int c)
    {
        long ab = LCM(a, b);
        long ac = LCM(a, c);
        long bc = LCM(b, c);
        long abc = LCM(a, bc);
        long left = 1;
        long right = 2_000_000_000;
        while (left < right)
        {
            long mid = left + (right - left) / 2;
            if (Count(mid) < n) { left = 1 + mid; }
            else { right = mid; }
        }
        return (int)left;

        long Count(long mid)
        => mid / a + mid / b + mid / c - mid / ab - mid / ac - mid / bc + mid / abc;

        static long GCD(long a, long b)
        {
            while (a != 0)
            {
                (a, b) = (b % a, a);
            }
            return b;
        }
        static long LCM(long a, long b) => a / GCD(a, b) * b;
    }
}