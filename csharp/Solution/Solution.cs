using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LenOfVDiagonal(int[][] grid)
    {
        int rows = grid.Length;
        int cols = grid[0].Length;
        var memo = new int[rows, cols, 4, 2];
        (int dr, int dc)[] dirs = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
        int res = 0;
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                for (int d = 0; d < 4 && grid[r][c] == 1; d++)
                {
                    res = int.Max(res, Dfs(r, c, d, 0, 1));
                }
            }
        }
        return res;

        int Dfs(int row, int col, int dir, int turn, int val)
        {
            if (row < 0 || rows <= row || col < 0 || cols <= col || grid[row][col] != val)
            {
                return 0;
            }
            if (memo[row, col, dir, turn] > 0) { return memo[row, col, dir, turn]; }
            int next_val = val < 2 ? 2 : 0;
            int nr = row + dirs[dir].dr;
            int nc = col + dirs[dir].dc;
            int res = Dfs(nr, nc, dir, turn, next_val);
            if (turn == 0)
            {
                int nd = (1 + dir) % 4;
                nr = row + dirs[nd].dr;
                nc = col + dirs[nd].dc;
                res = int.Max(res, Dfs(nr, nc, nd, 1, next_val));
            }
            res += 1;
            memo[row, col, dir, turn] = res;
            return res;
        }
    }
}