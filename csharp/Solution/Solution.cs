using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxIncreaseKeepingSkyline(int[][] grid)
    {
        int n = grid.Length;
        int[] rowmax = new int[n];
        int[] colmax = new int[n];
        for (int r = 0; r < n; r++)
        {
            for (int c = 0; c < n; c++)
            {
                rowmax[r] = int.Max(rowmax[r], grid[r][c]);
                colmax[c] = int.Max(colmax[c], grid[r][c]);
            }
        }
        int res = 0;
        for (int r = 0; r < n; r++)
        {
            for (int c = 0; c < n; c++)
            {
                res += int.Min(rowmax[r], colmax[c]) - grid[r][c];
            }
        }
        return res;
    }
}
