using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool SearchMatrix(int[][] matrix, int target)
    {
        int rows = matrix.Length;
        int cols = matrix[0].Length;
        int left = 0;
        int right = rows * cols - 1;
        while (left <= right)
        {
            int mid = left + (right - left) / 2;
            int r = mid / cols;
            int c = mid % cols;
            if (matrix[r][c] == target) { return true; }
            else if (matrix[r][c] < target) { left = 1 + mid; }
            else { right = mid - 1; }
        }
        return false;
    }
}