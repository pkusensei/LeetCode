using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int SmallestRepunitDivByK(int k)
    {
        if ((k & 1) == 0 || k % 5 == 0) { return -1; }
        int res = 1;
        int val = 1;
        while (val % k > 0)
        {
            res += 1;
            val = (10 * val + 1) % k;
        }
        return res;
    }
}

