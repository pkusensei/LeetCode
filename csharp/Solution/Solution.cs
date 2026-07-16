using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    int[] Vals
    {
        get
        {
            int[] vals = new int[38];
            vals[1] = 1;
            vals[2] = 1;
            for (int i = 3; i < 38; i++)
            {
                vals[i] = vals[i - 3] + vals[i - 2] + vals[i - 1];
            }
            return vals;
        }
    }

    public int Tribonacci(int n) => Vals[n];
}
