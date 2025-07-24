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
        int right = cols - 1;
        while (left < right)
        {
            int mid = left + (right - left + 1) / 2;
            if (matrix[0][mid] <= target) { left = mid; }
            else { right = mid - 1; }
        }
        if (matrix[0][left] == target) { return true; }
        int col = left;
        for (int c = col; c >= 0; c -= 1)
        {
            left = 0;
            right = rows - 1;
            while (left <= right)
            {
                int mid = left + (right - left) / 2;
                if (matrix[mid][c] < target) { left = 1 + mid; }
                else if (matrix[mid][c] > target) { right = mid - 1; }
                else { return true; }
            }
        }
        return false;
    }

    public bool WithLinearTime(int[][] mat, int target)
    {
        int rows = mat.Length;
        int cols = mat[0].Length;
        int r = 0;
        int c = cols - 1;
        while (r < rows && c >= 0)
        {
            int val = mat[r][c];
            if (val > target) { c -= 1; }
            else if (val < target) { r += 1; }
            else{ return true; }
        }
        return false;
    }
}