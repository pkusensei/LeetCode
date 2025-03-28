using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int[] MaxPoints(int[][] grid, int[] queries)
    {
        int rows = grid.Length;
        int cols = grid[0].Length;
        List<(int idx, int val)> qis = [.. queries.Select((val, idx) => (idx, val)).OrderBy(p => p.val)];
        PriorityQueue<(int, int), int> pq = new();
        pq.Enqueue((0, 0), grid[0][0]);
        bool[,] seen = new bool[rows, cols];
        seen[0, 0] = true;
        int[] res = new int[queries.Length];
        int count = 0;
        foreach (var (idx, val) in qis)
        {
            while (pq.TryPeek(out _, out int p) && p < val)
            {
                count += 1;
                (int r, int c) = pq.Dequeue();
                foreach (var (dr, dc) in new[] { (0, 1), (0, -1), (-1, 0), (1, 0) })
                {
                    int nr = r + dr;
                    int nc = c + dc;
                    if (0 <= nr && nr < rows && 0 <= nc && nc < cols && !seen[nr, nc])
                    {
                        seen[nr, nc] = true;
                        pq.Enqueue((nr, nc), grid[nr][nc]);
                    }
                }
            }
            res[idx] = count;
        }
        return res;
    }
}
