using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[][] SortMatrix(int[][] grid)
    {
        int n = grid.Length;
        List<int> curr;
        for (int col = 1; col < n; col++)
        {
            curr = [];
            for (int r = 0; r < n - col; r++)
            {
                curr.Add(grid[r][col + r]);
            }
            curr.Sort();
            for (int r = 0; r < n - col; r++)
            {
                grid[r][r + col] = curr[r];
            }
        }
        for (int row = 0; row < n; row++)
        {
            curr = [];
            for (int c = 0; c < n - row; c++)
            {
                curr.Add(grid[row + c][c]);
            }
            curr.Sort((a, b) => b.CompareTo(a));
            for (int c = 0; c < n - row; c++)
            {
                grid[row + c][c] = curr[c];
            }
        }
        return grid;
    }
}