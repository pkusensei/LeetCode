using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindKthNumber(int n, int k)
    {
        int curr = 1;
        k -= 1;
        while (k > 0)
        {
            // try moving to 1+curr
            var steps = CountSteps(curr);
            if (steps <= k)
            { // success!
                curr += 1;
                k -= steps;
            }
            else
            { // impossible! go deeper on this tree
                curr *= 10;
                k -= 1;
            }
        }
        return curr;

        int CountSteps(long curr)
        {
            long num1 = curr;
            long num2 = 1 + curr; // upper bound for this subtree
            long res = 0;
            while (num1 <= n)
            {
                res += Math.Min(1 + n, num2) - num1;
                num1 *= 10;
                num2 *= 10;
            }
            return (int)res;
        }
    }
}
