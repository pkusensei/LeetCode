using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxDistance(int[][] grid)
    {
        ReadOnlySpan<(int, int)> D = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        int n = grid.Length;
        bool[,] seen = new bool[n, n];
        Queue<(int row, int col)> queue = [];
        for (int r = 0; r < n; r++)
        {
            for (int c = 0; c < n; c++)
            {
                if (grid[r][c] == 1)
                {
                    seen[r, c] = true;
                    queue.Enqueue((r, c));
                }
            }
        }
        if (n * n == queue.Count) { return -1; }
        int dist = 0;
        while (queue.Count > 0)
        {
            dist += 1;
            int len = queue.Count;
            for (int _ = 0; _ < len; _++)
            {
                (int row, int col) = queue.Dequeue();
                foreach (var (dr, dc) in D)
                {
                    int nr = row + dr;
                    int nc = col + dc;
                    if (0 <= nr && nr < n && 0 <= nc && nc < n && !seen[nr, nc])
                    {
                        seen[nr, nc] = true;
                        queue.Enqueue((nr, nc));
                    }
                }
            }
        }
        return dist - 1;
    }
}
