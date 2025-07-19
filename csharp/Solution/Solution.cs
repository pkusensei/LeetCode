using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CalculateMinimumHP(int[][] dungeon)
    {
        int rows = dungeon.Length;
        int cols = dungeon[0].Length;
        int[,] dp = new int[rows, cols];
        dp[rows - 1, cols - 1] = int.Max(1, 1 - dungeon[rows - 1][cols - 1]);
        for (int row = rows - 2; row >= 0; row -= 1)
        {
            int c = cols - 1;
            dp[row, c] = int.Max(1, dp[1 + row, c] - dungeon[row][c]);
        }
        for (int col = cols - 2; col >= 0; col -= 1)
        {
            int r = rows - 1;
            dp[r, col] = int.Max(1, dp[r, 1 + col] - dungeon[r][col]);
        }
        for (int r = rows - 2; r >= 0; r -= 1)
        {
            for (int c = cols - 2; c >= 0; c -= 1)
            {
                int right = int.Max(1, dp[r, 1 + c] - dungeon[r][c]);
                int down = int.Max(1, dp[1 + r, c] - dungeon[r][c]);
                dp[r, c] = int.Min(right, down);
            }
        }
        return dp[0, 0];
    }
}
