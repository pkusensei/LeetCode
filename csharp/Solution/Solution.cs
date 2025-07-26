using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class NumMatrix
{
    public NumMatrix(int[][] matrix)
    {
        int rows = matrix.Length;
        int cols = matrix[0].Length;
        Prefix = new int[1 + rows, 1 + cols];
        for (int r = 0; r < rows; r++)
        {
            int sum = 0;
            for (int c = 0; c < cols; c++)
            {
                sum += matrix[r][c];
                Prefix[1 + r, 1 + c] = sum + Prefix[r, 1 + c];
            }
        }
    }

    int[,] Prefix { get; }

    public int SumRegion(int row1, int col1, int row2, int col2)
    {
        row2 += 1;
        col2 += 1;
        return Prefix[row2, col2] + Prefix[row1, col1] - Prefix[row2, col1] - Prefix[row1, col2];
    }
}