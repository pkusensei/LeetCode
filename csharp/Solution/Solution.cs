using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CutOffTree(IList<IList<int>> forest)
    {
        int rows = forest.Count;
        int cols = forest[0].Count;
        PriorityQueue<Coord, int> trees = new(rows * cols);
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                if (forest[r][c] > 1)
                {
                    trees.Enqueue(new(r, c), forest[r][c]);
                }
            }
        }
        Coord spot = new(0, 0);
        int res = 0;
        while (trees.TryDequeue(out var dest, out _))
        {
            int v = Bfs(spot, dest);
            if (v < 0) { return -1; }
            res += v;
            spot = dest;
        }
        return res;

        int Bfs(Coord start, Coord dest)
        {
            Queue<(Coord, int num)> queue = [];
            bool[,] seen = new bool[rows, cols];
            queue.Enqueue((start, 0));
            seen[start.Row, start.Col] = true;
            while (queue.TryDequeue(out var item))
            {
                (Coord curr, int step) = item;
                if (curr == dest) { return step; }
                foreach (var (dr, dc) in new[] { (-1, 0), (1, 0), (0, -1), (0, 1) })
                {
                    Coord next = new(curr.Row + dr, curr.Col + dc);
                    if (0 <= next.Row && next.Row < rows && 0 <= next.Col && next.Col < cols
                    && forest[next.Row][next.Col] > 0 && !seen[next.Row, next.Col])
                    {
                        seen[next.Row, next.Col] = true;
                        queue.Enqueue((next, 1 + step));
                    }
                }
            }
            return -1;
        }
    }

    readonly record struct Coord(int Row, int Col);
}