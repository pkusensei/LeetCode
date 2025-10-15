using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxAreaOfIsland(int[][] grid)
    {
        int rows = grid.Length;
        int cols = grid[0].Length;
        DSU dsu = new(rows * cols);
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                if (grid[r][c] == 1)
                {
                    dsu.Size[r * cols + c] = 1;
                }
            }
        }
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                if (grid[r][c] == 1)
                {
                    foreach (var (dr, dc) in new[] { (-1, 0), (0, -1), (1, 0), (0, 1) })
                    {
                        int nr = r + dr;
                        int nc = c + dc;
                        if (0 <= nr && nr < rows && 0 <= nc && nc < cols && grid[nr][nc] == 1)
                        {
                            dsu.Union(r * cols + c, nr * cols + nc);
                        }
                    }
                }
            }
        }
        return dsu.Size.Max();
    }
}

readonly struct DSU
{
    public DSU(int n)
    {
        Parent = [.. Enumerable.Range(0, n)];
        Size = new int[n];
    }

    public int[] Parent { get; }
    public int[] Size { get; }

    public int Find(int v)
    {
        if (Parent[v] != v) { Parent[v] = Find(Parent[v]); }
        return Parent[v];
    }

    public void Union(int x, int y)
    {
        int rx = Find(x);
        int ry = Find(y);
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
}
