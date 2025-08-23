using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinimumSum(int[][] grid)
    {
        const int M = 1_000;
        int rows = grid.Length;
        int cols = grid[0].Length;
        return Dfs(0, 0, rows, cols, 2);

        int Dfs(int minr, int minc, int maxr, int maxc, int cuts)
        {
            if (cuts == 0) { return MinArea(minr, minc, maxr, maxc); }
            int res = M;
            for (int r = 1 + minr; r < maxr; r++)
            {
                var v1 = Dfs(minr, minc, r, maxc, cuts - 1);
                var v2 = MinArea(r, minc, maxr, maxc);
                res = int.Min(res, v1 + v2);
                v1 = MinArea(minr, minc, r, maxc);
                v2 = Dfs(r, minc, maxr, maxc, cuts - 1);
                res = int.Min(res, v1 + v2);
            }
            for (int c = 1 + minc; c < maxc; c++)
            {
                var v1 = Dfs(minr, minc, maxr, c, cuts - 1);
                var v2 = MinArea(minr, c, maxr, maxc);
                res = int.Min(res, v1 + v2);
                v1 = MinArea(minr, minc, maxr, c);
                v2 = Dfs(minr, c, maxr, maxc, cuts - 1);
                res = int.Min(res, v1 + v2);
            }
            return res;
        }

        int MinArea(int minr, int minc, int maxr, int maxc)
        {
            int res_minr = M;
            int res_minc = M;
            int res_maxr = 0;
            int res_maxc = 0;
            for (int r = minr; r < maxr; r++)
            {
                for (int c = minc; c < maxc; c++)
                {
                    if (grid[r][c] == 1)
                    {
                        res_minr = int.Min(res_minr, r);
                        res_minc = int.Min(res_minc, c);
                        res_maxr = int.Max(res_maxr, r);
                        res_maxc = int.Max(res_maxc, c);
                    }
                }
            }
            int res = (res_maxr - res_minr + 1) * (res_maxc - res_minc + 1);
            return res;
        }
    }
}