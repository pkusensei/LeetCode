using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public void Rotate(int[][] matrix)
    {
        int n = matrix.Length;
        for (int r = 0; r < n; r++)
        {
            for (int c = 0; c < r; c++)
            {
                (matrix[r][c], matrix[c][r]) = (matrix[c][r], matrix[r][c]);
            }
        }
        for (int r = 0; r < n; r++)
        {
            for (int c = 0; c < n / 2; c++)
            {
                (matrix[r][c], matrix[r][n - 1 - c]) = (matrix[r][n - 1 - c], matrix[r][c]);
            }
        }
    }
}
