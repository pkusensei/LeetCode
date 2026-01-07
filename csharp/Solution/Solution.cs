using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int ConsecutiveNumbersSum(int n)
    {
        // a + a+1 + a+2 + .. + a+i
        // a*(1+i) + (1+2+..+i)
        int res = 0;
        for (int i = 0; i * (1 + i) < 2 * n; i++)
        {
            int d = n - i * (1 + i) / 2;
            if (d % (1 + i) == 0) { res += 1; }
        }
        return res;
    }
}
