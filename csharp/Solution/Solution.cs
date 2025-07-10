using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int UniquePathsWithObstacles(int[][] obstacleGrid)
    {
        int m = obstacleGrid.Length;
        int n = obstacleGrid[0].Length;
        int[,] dp = new int[m, n];
        dp[0, 0] = 1 - obstacleGrid[0][0];
        for (int c = 1; c < n; c++)
        {
            dp[0, c] = obstacleGrid[0][c] == 1 ? 0 : dp[0, c - 1];
        }
        for (int r = 1; r < m; r++)
        {
            dp[r, 0] = obstacleGrid[r][0] == 1 ? 0 : dp[r - 1, 0];
        }
        for (int r = 1; r < m; r++)
        {
            for (int c = 1; c < n; c++)
            {
                dp[r, c] = obstacleGrid[r][c] == 1 ? 0 : dp[r - 1, c] + dp[r, c - 1];
            }
        }
        return dp[m - 1, n - 1];
    }
}