using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long MinOperations(int[][] queries)
    {
        long res = 0;
        foreach (var q in queries)
        {
            res += (1 + Count(q[1]) - Count(q[0] - 1)) / 2;
        }
        return res;

        static long Count(int num)
        {
            long res = 0;
            long p = 1;
            long val = 1;
            for (; 4 * val <= num; val *= 4)
            {
                res += (4 * val - val) * p; // [val, 4val) needs p operations
                p += 1;
            }
            return res + (num - val + 1) * p;
        }
    }
}
