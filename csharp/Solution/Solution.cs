using System.Buffers;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinTimeToReach(int[][] moveTime)
    {
        int rows = moveTime.Length;
        int cols = moveTime[0].Length;
        int[,] dists = new int[rows, cols];
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                dists[r, c] = int.MaxValue;
            }
        }
        PriorityQueue<(int, int, int), int> pq = new();
        pq.Enqueue((0, 0, 1), 0);
        while (pq.TryDequeue(out var item, out var dist))
        {
            (var r, var c, var cost) = item;
            if (r == rows - 1 && c == cols - 1) { return dist; }
            if (dist > dists[r, c]) { continue; }
            foreach (var (dr, dc) in new[] { (0, -1), (0, 1), (-1, 0), (1, 0) })
            {
                int nr = r + dr;
                int nc = c + dc;
                if (0 <= nr && nr < rows && 0 <= nc && nc < cols)
                {
                    int next_d = cost + Math.Max(dist, moveTime[nr][nc]);
                    if (next_d < dists[nr, nc])
                    {
                        dists[nr, nc] = next_d;
                        pq.Enqueue((nr, nc, 3 - cost), next_d);
                    }
                }
            }
        }
        return dists[rows - 1, cols - 1];
    }
}
