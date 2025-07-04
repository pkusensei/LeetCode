using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Reverse(int x)
    {
        bool is_neg = x < 0;
        if (is_neg) { x = -x; }
        List<int> ds = [];
        while (x > 0)
        {
            ds.Add(x % 10);
            x /= 10;
        }
        try
        {
            int res = 0;
            checked
            {
                foreach (var d in ds)
                {
                    res = 10 * res + d;
                }
            }
            return is_neg ? -res : res;
        }
        catch (OverflowException)
        {
            return 0;
        }
    }
}
