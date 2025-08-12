using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindKthNumber(int n, int k)
    {
        int res = 1;
        k -= 1;
        while (k > 0)
        {
            int steps = Step(res);
            if (steps <= k)
            {
                res += 1;
                k -= steps;
            }
            else
            {
                k -= 1;
                res *= 10;
            }
        }
        return res;

        int Step(int curr)
        {
            long n1 = curr;
            long n2 = 1 + curr;
            long steps = 0;
            while (n1 <= n)
            {
                steps += long.Min(1 + n, n2) - n1;
                n1 *= 10;
                n2 *= 10;
            }
            return (int)steps;
        }
    }
}