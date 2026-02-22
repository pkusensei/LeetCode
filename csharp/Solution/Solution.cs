using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int BinaryGap(int n)
    {
        int res = 0;
        int prev = -1;
        int i = 0;
        while (n > 0)
        {
            if ((n & 1) == 1)
            {
                if (prev > -1) { res = int.Max(res, i - prev); }
                prev = i;
            }
            i += 1;
            n >>= 1;
        }
        return res;
    }
}