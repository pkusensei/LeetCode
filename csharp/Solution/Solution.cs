using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> PacificAtlantic(int[][] heights)
    {
        int rows = heights.Length;
        int cols = heights[0].Length;
        HashSet<(int, int)> a = [];
        Queue<(int, int)> queue = [];
        for (int i = 0; i < cols; i++)
        {
            a.Add((0, i));
            queue.Enqueue((0, i));
        }
        for (int i = 1; i < rows; i++)
        {
            a.Add((i, 0));
            queue.Enqueue((i, 0));
        }
        a = Bfs(queue, a);
        queue.Clear();
        HashSet<(int, int)> b = [];
        for (int i = 0; i < cols; i++)
        {
            b.Add((rows - 1, i));
            queue.Enqueue((rows - 1, i));
        }
        for (int i = 0; i < rows - 1; i++)
        {
            b.Add((i, cols - 1));
            queue.Enqueue((i, cols - 1));
        }
        b = Bfs(queue, b);
        return [.. a.Intersect(b).Select(v => new[] { v.Item1, v.Item2 })];

        HashSet<(int, int)> Bfs(Queue<(int, int)> queue, HashSet<(int, int)> set)
        {
            Span<(int, int)> deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            while (queue.TryDequeue(out var item))
            {
                (int r, int c) = item;
                foreach (var (dr, dc) in deltas)
                {
                    int nr = r + dr;
                    int nc = c + dc;
                    if (0 <= nr && nr < rows && 0 <= nc && nc < cols
                    && heights[nr][nc] >= heights[r][c] && set.Add((nr, nc)))
                    {
                        queue.Enqueue((nr, nc));
                    }
                }
            }
            return set;
        }
    }
}
