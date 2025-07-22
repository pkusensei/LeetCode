using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaximalSquare(char[][] matrix)
    {
        int rows = matrix.Length;
        int cols = matrix[0].Length;
        int[,] dp = new int[rows, cols];
        int res = 0;
        for (int r = 0; r < rows; r++)
        {
            if (matrix[r][0] == '1')
            {
                dp[r, 0] = 1;
                res = 1;
            }
        }
        for (int c = 0; c < cols; c++)
        {
            if (matrix[0][c] == '1')
            {
                dp[0, c] = 1;
                res = 1;
            }
        }
        for (int r = 1; r < rows; r++)
        {
            for (int c = 1; c < cols; c++)
            {
                if (matrix[r][c] == '1')
                {
                    dp[r, c] = 1 + int.Min(dp[r - 1, c - 1], int.Min(dp[r - 1, c], dp[r, c - 1]));
                    res = int.Max(res, dp[r, c]);
                }
            }
        }
        return res * res;
    }
}