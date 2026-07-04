using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinScore(int n, int[][] roads)
    {
        DSU dsu = new(n);
        foreach (var item in roads)
        {
            dsu.Union(item[0] - 1, item[1] - 1);
        }
        int res = int.MaxValue;
        foreach (var item in roads)
        {
            if (dsu.Find(0) == dsu.Find(item[0] - 1))
            {
                res = int.Min(res, item[2]);
            }
        }
        return res;
    }
}

public readonly struct DSU
{
    public DSU(int n)
    {
        Parent = [.. Enumerable.Range(0, n)];
        Rank = new int[n];
    }

    public int[] Parent { get; }
    public int[] Rank { get; }

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
        if (Rank[rx] < Rank[ry]) { Parent[rx] = ry; }
        else if (Rank[rx] > Rank[ry]) { Parent[ry] = rx; }
        else
        {
            Rank[rx] += 1;
            Parent[ry] = rx;
        }
    }
}