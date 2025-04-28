using System.Buffers;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] MinimumCost(int n, int[][] edges, int[][] query)
    {
        var dsu = new DSU(n);
        foreach (var item in edges)
        {
            dsu.Union(item[0], item[1], item[2]);
        }
        return [.. query.Select(q =>
        {
            int x = dsu.Find(q[0]);
            int y = dsu.Find(q[1]);
            return x == y ? dsu.Vals[x] : -1;
        })];
    }
}

class DSU
{
    public int[] Parent { get; }
    public int[] Rank { get; }
    public int[] Vals { get; }

    public DSU(int n)
    {
        Parent = [.. Enumerable.Range(0, n)];
        Rank = new int[n];
        Vals = new int[n];
        Array.Fill(Vals, int.MaxValue);
    }

    public int Find(int v)
    {
        if (Parent[v] != v) { Parent[v] = Find(Parent[v]); }
        return Parent[v];
    }

    public void Union(int x, int y, int val)
    {
        int rx = Find(x);
        int ry = Find(y);
        if (rx == ry)
        {
            Vals[rx] &= val;
            return;
        }
        if (Rank[rx] < Rank[ry])
        {
            Parent[rx] = ry;
            Vals[ry] &= val;
            Vals[ry] &= Vals[rx];
        }
        else if (Rank[rx] == Rank[ry])
        {
            Parent[ry] = rx;
            Rank[rx] += 1;
            Vals[rx] &= val;
            Vals[rx] &= Vals[ry];
        }
        else
        {
            Parent[ry] = rx;
            Vals[rx] &= val;
            Vals[rx] &= Vals[ry];
        }
    }
}
