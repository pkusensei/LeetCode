using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int LargestIsland(int[][] grid)
    {
        (int, int)[] deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        int n = grid.Length;
        List<(int row, int col)> zeros = [];
        foreach (var (ri, row) in grid.Select((v, i) => (i, v)))
        {
            foreach (var (ci, val) in row.Select((v, i) => (i, v)))
            {
                if (val == 0) { zeros.Add((ri, ci)); }
            }
        }
        if (zeros.Count == 0) { return n * n; }
        if (zeros.Count == n * n) { return 1; }
        bool[,] seen = new bool[n, n];
        DSU dsu = new(n * n);
        int max_island = 0;
        foreach (var (ri, row) in grid.Select((v, i) => (i, v)))
        {
            foreach (var (ci, val) in row.Select((v, i) => (i, v)))
            {
                if (val == 1 && !seen[ri, ci]) { max_island = Math.Max(max_island, BFS(ri, ci)); }
            }
        }
        int res = 1 + max_island;
        foreach (var (row, col) in zeros)
        {
            HashSet<int> islands = [];
            foreach (var (dr, dc) in deltas)
            {
                var nr = row + dr;
                var nc = col + dc;
                if (0 <= nr && nr < n && 0 <= nc && nc < n && grid[nr][nc] == 1)
                {
                    islands.Add(dsu.Find(nr * n + nc));
                }
            }
            var curr = 1 + islands.Select(v => dsu.SizeOf(v)).Sum();
            res = Math.Max(res, curr);
        }
        return res;

        int BFS(int row, int col)
        {
            Queue<(int row, int col)> queue = [];
            queue.Enqueue((row, col));
            seen[row, col] = true;
            while (queue.TryDequeue(out var item))
            {
                (var cr, var cc) = item;
                dsu.Union(row * n + col, cr * n + cc);
                foreach (var (dr, dc) in deltas)
                {
                    var nr = cr + dr;
                    var nc = cc + dc;
                    if (0 <= nr && nr < n && 0 <= nc && nc < n && !seen[nr, nc] && grid[nr][nc] == 1)
                    {
                        seen[nr, nc] = true;
                        queue.Enqueue((nr, nc));
                    }
                }
            }
            return dsu.SizeOf(row * n + col);
        }
    }
}

class DSU
{
    public DSU(int n)
    {
        Parent = [.. Enumerable.Range(0, n)];
        Size = new int[n];
        Array.Fill(Size, 1);
    }

    public int[] Parent { get; }
    public int[] Size { get; }

    public int Find(int x)
    {
        if (Parent[x] != x) { Parent[x] = Find(Parent[x]); }
        return Parent[x];
    }

    public void Union(int x, int y)
    {
        (var rx, var ry) = (Find(x), Find(y));
        if (rx == ry) { return; }
        if (Size[rx] < Size[ry])
        {
            Size[ry] += Size[rx];
            Parent[rx] = ry;
        }
        else
        {
            Size[rx] += Size[ry];
            Parent[ry] = rx;
        }
    }

    public int SizeOf(int x) => Size[Find(x)];
}
