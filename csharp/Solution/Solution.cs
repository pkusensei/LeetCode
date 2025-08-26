using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int AreaOfMaxDiagonal(int[][] dimensions)
    {
        int max_d = 0;
        int res = 0;
        foreach (var dim in dimensions)
        {
            int d = dim[0] * dim[0] + dim[1] * dim[1];
            if (max_d < d)
            {
                max_d = d;
                res = dim[0] * dim[1];
            }
            else if (max_d == d)
            {
                res = int.Max(res, dim[0] * dim[1]);
            }
        }
        return res;
    }
}