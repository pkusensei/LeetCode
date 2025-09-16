using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool JudgeSquareSum(int c)
    {
        long a = 0;
        long b = (long)Math.Sqrt(c);
        while (a <= b)
        {
            long sum = a * a + b * b;
            if (sum > c) { b -= 1; }
            else if (sum < c) { a += 1; }
            else { return true; }
        }
        return false;
    }
}