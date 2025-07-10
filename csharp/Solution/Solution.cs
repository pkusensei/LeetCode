using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinPathSum(int[][] grid)
    {
        int rows = grid.Length;
        int cols = grid[0].Length;
        int[,] dp = new int[rows, cols];
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                dp[r, c] = int.MaxValue;
            }
        }
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                if (r == 0 && c == 0) { dp[0, 0] = grid[0][0]; }
                else if (r > 0 && c > 0)
                {
                    dp[r, c] = Math.Min(dp[r, c],
                                    Math.Min(grid[r][c] + dp[r - 1, c], grid[r][c] + dp[r, c - 1]));
                }
                else if (r > 0)
                {
                    dp[r, c] = grid[r][c] + dp[r - 1, c];
                }
                else
                {
                    dp[r, c] = grid[r][c] + dp[r, c - 1];
                }
            }
        }
        return dp[rows - 1, cols - 1];
    }
}