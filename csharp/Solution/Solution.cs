using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long DistributeCandies(int n, int limit)
    {
        long res = 0;
        // Fix a
        for (int a = 0; a <= Math.Min(n, limit); a++)
        {
            // b = 0;
            int c_max = Math.Min(n - a, limit);
            // b = limit
            int c_min = Math.Max(n - a - limit, 0);
            res += Math.Max(c_max - c_min + 1, 0);
        }
        return res;
    }

    public long PIE(int n, int limit)
    {
        // All possible distribution 
        // minus: at least one receives more than limit
        // plus: at least two receive more than limit
        // minus: all receive more than limit
        return NChoose2(2 + n)
               - 3 * NChoose2(n + 1 - limit)
               + 3 * NChoose2(n - (1 + limit) * 2 + 2)
               - NChoose2(n - 3 * (1 + limit) + 2);

        long NChoose2(long x) => x <= 1 ? 0 : x * (x - 1) / 2;
    }
}
