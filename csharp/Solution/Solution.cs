using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] ProductQueries(int n, int[][] queries)
    {
        List<int> ps = [];
        for (int p = 0; p < 32; p++)
        {
            if (((n >> p) & 1) == 1) { ps.Add((int)Math.Pow(2, p)); }
        }
        return queries.Select(q =>
        {
            long res = 1;
            for (int i = q[0]; i <= q[1]; i++)
            {
                res = res * ps[i] % 1_000_000_007;
            }
            return (int)res;
        }).ToArray();
    }
}