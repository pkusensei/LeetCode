using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinFallingPathSum(int[][] matrix)
    {
        int n = matrix.Length;
        for (int r = 1; r < n; r++)
        {
            for (int c = 0; c < n; c++)
            {
                int left = int.Max(0, c - 1);
                int right = int.Min(1 + c, n - 1);
                matrix[r][c] += matrix[r - 1][left..(1 + right)].Min();
            }
        }
        return matrix.Last().Min();
    }
}