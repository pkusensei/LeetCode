using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountNumbersWithUniqueDigits(int n)
    {
        if (n < 1) { return 1; }
        int res = 9;
        int v = 9;
        for (int _ = 1; _ < n; _++)
        {
            res *= v;
            v -= 1;
        }
        return res + CountNumbersWithUniqueDigits(n - 1);
    }
}