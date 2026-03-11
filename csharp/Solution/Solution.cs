using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinMalwareSpread(int[][] graph, int[] initial)
    {
        int n = graph.Length;
        DSU dsu = new(n);
        for (int i1 = 0; i1 < n; i1++)
        {
            for (int i2 = 1 + i1; i2 < n; i2++)
            {
                if (graph[i1][i2] == 1) { dsu.Union(i1, i2); }
            }
        }
        Dictionary<int, List<int>> roots = [];
        foreach (int i in initial)
        {
            int root = dsu.Find(i);
            if (!roots.TryAdd(root, [i])) { roots[root].Add(i); }
        }
        int res = n;
        int max_size = 0;
        foreach (var (root, nodes) in roots)
        {
            if (nodes.Count > 1) { continue; }
            int size = dsu.Size[root];
            if (size > max_size)
            {
                max_size = size;
                res = nodes[0];
            }
            else if (size == max_size)
            {
                res = int.Min(res, nodes[0]);
            }
        }
        return res == n ? initial.Min() : res;
    }
}

readonly struct DSU
{
    public int[] Parent { get; }
    public int[] Size { get; }

    public DSU(int n)
    {
        Parent = [.. Enumerable.Range(0, n)];
        Size = [.. Enumerable.Repeat(1, n)];
    }

    public readonly int Find(int v)
    {
        if (Parent[v] != v) { Parent[v] = Find(Parent[v]); }
        return Parent[v];
    }

    public readonly void Union(int x, int y)
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