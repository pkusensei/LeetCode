using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int TrailingZeroes(int n)
    {
        int div = 5;
        int res = 0;
        while (n / div > 0)
        {
            res += n / div;
            div *= 5;
        }
        // first count in 5, 10, 15, .., 25, 30, ..
        // then count 25, 50, 75, ..
        // 125, 250, ..
        return res;
    }
}
