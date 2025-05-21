using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public void SetZeroes(int[][] matrix)
    {
        int rows = matrix.Length;
        int cols = matrix[0].Length;
        bool first_row = false;
        bool first_col = false;
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                if (matrix[r][c] == 0)
                {
                    matrix[0][c] = 0;
                    matrix[r][0] = 0;
                    first_row |= r == 0;
                    first_col |= c == 0;
                }
            }
        }
        for (int c = 1; c < cols; c++)
        {
            if (matrix[0][c] == 0)
            {
                for (int r = 0; r < rows; r++)
                {
                    matrix[r][c] = 0;
                }
            }
        }
        for (int r = 1; r < rows; r++)
        {
            if (matrix[r][0] == 0)
            {
                for (int c = 0; c < cols; c++)
                {
                    matrix[r][c] = 0;
                }
            }
        }
        if (first_row) { Array.Fill(matrix[0], 0); }
        if (first_col)
        {
            for (int r = 0; r < rows; r++)
            {
                matrix[r][0] = 0;
            }
        }
    }
}
