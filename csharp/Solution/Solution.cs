using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MinimumTime(int[][] grid)
    {
        int rows = grid.Length;
        int cols = grid[0].Length;
        if (grid[1][0] > 1 && grid[0][1] > 1) { return -1; }
        var pq = new PriorityQueue<(int, int), int>();
        pq.Enqueue((0, 0), 0);
        var seen = new bool[rows, cols];
        seen[0, 0] = true;
        while (pq.TryDequeue(out var item, out var time))
        {
            (int r, int c) = item;
            if (r == rows - 1 && c == cols - 1) { return time; }
            foreach (var (dr, dc) in new[] { (1, 0), (-1, 0), (0, 1), (0, -1) })
            {
                int nr = r + dr;
                int nc = c + dc;
                if (0 <= nr && nr < rows && 0 <= nc && nc < cols && !seen[nr, nc])
                {
                    seen[nr, nc] = true;
                    int t = grid[nr][nc];
                    if (t <= 1 + time) { t = 1 + time; }
                    else if (((t - time) & 1) == 0) { t += 1; }
                    pq.Enqueue((nr, nc), t);
                }
            }
        }
        return -1;
    }
}
