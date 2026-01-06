using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LargestIsland(int[][] grid)
    {
        int n = grid.Length;
        DSU dsu = new(n * n);
        for (int r = 0; r < n; r++)
        {
            for (int c = 0; c < n; c++)
            {
                if (grid[r][c] == 1)
                {
                    if (1 + r < n && grid[1 + r][c] == 1)
                    {
                        dsu.Union(r * n + c, (1 + r) * n + c);
                    }
                    if (1 + c < n && grid[r][1 + c] == 1)
                    {
                        dsu.Union(r * n + c, r * n + 1 + c);
                    }
                }
            }
        }
        ReadOnlySpan<(int dr, int dc)> D = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        int res = dsu.Size.Max();
        for (int r = 0; r < n; r++)
        {
            for (int c = 0; c < n; c++)
            {
                if (grid[r][c] == 1) { continue; }
                HashSet<int> roots = [];
                foreach (var (dr, dc) in D)
                {
                    int nr = r + dr;
                    int nc = c + dc;
                    if (0 <= nr && nr < n && 0 <= nc && nc < n && grid[nr][nc] == 1)
                    {
                        int root = dsu.Find(nr * n + nc);
                        roots.Add(root);
                    }
                }
                int curr = 1 + roots.Sum(r => dsu.Size[r]);
                res = int.Max(res, curr);
            }
        }
        return res;
    }
}

struct DSU
{
    public DSU(int n)
    {
        Parent = [.. Enumerable.Range(0, n)];
        Size = [.. Enumerable.Repeat(1, n)];
    }

    public int[] Parent;
    public int[] Size;

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