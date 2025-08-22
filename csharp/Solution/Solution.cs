using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinimumArea(int[][] grid)
    {
        int rmin = int.MaxValue;
        int cmin = int.MaxValue;
        int rmax = 0;
        int cmax = 0;
        for (int r = 0; r < grid.Length; r++)
        {
            for (int c = 0; c < grid[0].Length; c++)
            {
                if (grid[r][c] == 1)
                {
                    rmin = int.Min(rmin, r);
                    cmin = int.Min(cmin, c);
                    rmax = int.Max(rmax, r);
                    cmax = int.Max(cmax, c);
                }
            }
        }
        return (rmax - rmin + 1) * (cmax - cmin + 1);
    }
}