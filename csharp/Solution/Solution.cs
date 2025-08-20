using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountSquares(int[][] matrix)
    {
        int rows = matrix.Length;
        int cols = matrix[0].Length;
        int[,] dp = new int[rows, cols];
        int res = 0;
        for (int i = 0; i < cols; i++)
        {
            dp[0, i] = matrix[0][i];
            res += dp[0, i];
        }
        for (int i = 1; i < rows; i++)
        {
            dp[i, 0] = matrix[i][0];
            res += dp[i, 0];
        }
        for (int r = 1; r < rows; r++)
        {
            for (int c = 1; c < cols; c++)
            {
                if (matrix[r][c] == 1)
                {
                    dp[r, c] = 1 + int.Min(dp[r - 1, c - 1], int.Min(dp[r - 1, c], dp[r, c - 1]));
                    res += dp[r, c];
                }
            }
        }
        return res;
    }

    public int WithBetterSpace(int[][] mat)
    {
        int rows = mat.Length;
        int cols = mat[0].Length;
        int prev = 0;
        int res = 0;
        int[] dp = new int[1 + cols];
        for (int r = 1; r <= rows; r++)
        {
            for (int c = 1; c <= cols; c++)
            {
                if (mat[r - 1][c - 1] == 1)
                {
                    int temp = dp[c];
                    dp[c] = 1 + int.Min(prev, int.Min(dp[c - 1], dp[c]));
                    prev = temp;
                    res += dp[c];
                }
                else
                {
                    dp[c] = 0;
                }
            }
        }
        return res;
    }
}