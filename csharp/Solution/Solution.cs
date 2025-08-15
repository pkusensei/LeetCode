using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsPowerOfFour(int n)
    {
        int p2 = 0;
        int val = 2;
        for (int _ = 0; _ < 16; _++)
        {
            p2 |= val;
            val <<= 2;
        }
        return n > 0 && (n & (n - 1)) == 0 && (n & p2) == 0;
    }
}