using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int TrapRainWater(int[][] heightMap)
    {
        var rows = heightMap.Length;
        var cols = heightMap[0].Length;
        var heap = new PriorityQueue<(int r, int c, int h), int>();
        var seen = new bool[rows, cols];
        foreach (var (r, row) in heightMap.Select((v, i) => (i, v)))
        {
            foreach (var (c, val) in row.Select((v, i) => (i, v)))
            {
                if (r == rows - 1 || c == cols - 1 || r == 0 || c == 0)
                {   //start from edges
                    heap.Enqueue((r, c, val), val);
                    seen[r, c] = true;
                }
            }
        }
        var res = 0;
        var height = 0;
        // pop "lowest" cell first
        while (heap.TryDequeue(out var item, out var _))
        {
            height = Math.Max(height, item.h);
            foreach (var (dr, dc) in new[] { (-1, 0), (1, 0), (0, -1), (0, 1) })
            {
                var nr = item.r + dr;
                var nc = item.c + dc;
                if (0 <= nr && nr < rows && 0 <= nc && nc < cols && !seen[nr, nc])
                {
                    seen[nr, nc] = true;
                    var h = heightMap[nr][nc];
                    heap.Enqueue((nr, nc, h), h);
                    res += Math.Max(0, height - h);
                }
            }
        }
        return res;
    }
}
