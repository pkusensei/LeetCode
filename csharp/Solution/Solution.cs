using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int ArrangeCoins(int n)
    {
        long left = 1;
        long right = int.MaxValue;
        while (left < right)
        {
            long mid = left + (right - left + 1) / 2;
            if (mid * (1 + mid) / 2 > n) { right = mid - 1; }
            else { left = mid; }
        }
        return (int)left;
    }
}