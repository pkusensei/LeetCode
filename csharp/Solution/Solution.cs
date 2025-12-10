using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] HitBricks(int[][] grid, int[][] hits)
    {
        ReadOnlySpan<(int, int)> dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        int rows = grid.Length;
        int cols = grid[0].Length;
        DSU dsu = new(1 + rows * cols);
        bool[,] dotted = new bool[rows, cols];
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                if (grid[r][c] == 1) { dotted[r, c] = true; }
            }
        }
        foreach (var h in hits)
        {
            dotted[h[0], h[1]] = false;
        }
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                int i1 = IdOf(r, c);
                if (dotted[r, c])
                {
                    if (r == 0) { dsu.Union(0, i1); }
                    foreach (var (dr, dc) in dirs)
                    {
                        (int nr, int nc) = (r + dr, c + dc);
                        if (0 <= nr && nr < rows && 0 <= nc && nc < cols && dotted[nr, nc])
                        {
                            dsu.Union(i1, IdOf(nr, nc));
                        }
                    }
                }
            }
        }
        List<int> res = new(hits.Length);
        foreach (var h in hits.Reverse())
        {
            (int r, int c) = (h[0], h[1]);
            if (grid[r][c] == 0)
            {
                res.Add(0);
                continue;
            }
            int temp = dsu.Top;
            int i1 = IdOf(r, c);
            if (r == 0) { dsu.Union(0, i1); }
            foreach (var (dr, dc) in dirs)
            {
                (int nr, int nc) = (r + dr, c + dc);
                if (0 <= nr && nr < rows && 0 <= nc && nc < cols && dotted[nr, nc])
                {
                    dsu.Union(i1, IdOf(nr, nc));
                }
            }
            dotted[r, c] = true;
            res.Add(int.Max(0, dsu.Top - temp - 1));
        }
        return [.. res.AsEnumerable().Reverse()];

        int IdOf(int row, int col) => 1 + row * cols + col;
    }
}

readonly struct DSU
{
    public DSU(int n)
    {
        Parent = [.. Enumerable.Range(0, n)];
        Size = [.. Enumerable.Repeat(1, n)];
        Size[0] = 0;
    }

    public int[] Parent { get; }
    public int[] Size { get; }
    public int Top => Size[Find(0)];

    public int Find(int v)
    {
        if (Parent[v] != v) { Parent[v] = Find(Parent[v]); }
        return Parent[v];
    }

    public void Union(int x, int y)
    {
        (int rx, int ry) = (Find(x), Find(y));
        if (rx == ry) { return; }
        if (Size[rx] < Size[ry])
        {
            Parent[rx] = ry;
            Size[ry] += Size[rx];
        }
        else
        {
            Parent[ry] = rx;
            Size[rx] += Size[ry];
        }
    }
}
