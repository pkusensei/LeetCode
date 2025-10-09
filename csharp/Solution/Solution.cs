using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] FindRedundantConnection(int[][] edges)
    {
        DSU dsu = new(edges.Length);
        foreach (var e in edges)
        {
            if (!dsu.Union(e[0] - 1, e[1] - 1)) { return e; }
        }
        return null;
    }
}

readonly struct DSU
{
    public DSU(int n)
    {
        Parent = [.. Enumerable.Range(0, n)];
        Rank = new int[n];
    }

    int[] Parent { get; }
    int[] Rank { get; }

    public int Find(int v)
    {
        if (Parent[v] != v) { Parent[v] = Find(Parent[v]); }
        return Parent[v];
    }

    public bool Union(int x, int y)
    {
        int rx = Find(x);
        int ry = Find(y);
        if (rx == ry) { return false; }
        if (Rank[rx] < Rank[ry]) { Parent[rx] = ry; }
        else if (Rank[rx] > Rank[ry]) { Parent[ry] = rx; }
        else
        {
            Parent[ry] = rx;
            Rank[rx] += 1;
        }
        return true;
    }
}