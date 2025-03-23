using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MagnificentSets(int n, int[][] edges)
    {
        var dsu = new DSU(n);
        List<List<int>> adj = [.. Enumerable.Range(0, n).Select(_ => new List<int>())];
        foreach (var item in edges)
        {
            int a = item[0] - 1;
            int b = item[1] - 1;
            dsu.Union(a, b);
            adj[a].Add(b);
            adj[b].Add(a);
        }
        Dictionary<int, int> components = [];
        for (int node = 0; node < n; node++)
        {
            int v = BFS(node);
            if (v == -1) { return -1; }
            int root = dsu.Find(node);
            if (!components.TryAdd(root, v))
            {
                components[root] = Math.Max(components[root], v);
            }
        }
        return components.Values.Sum();

        int BFS(int start)
        {
            Queue<int> queue = [];
            queue.Enqueue(start);
            Span<int> seen = stackalloc int[n];
            seen.Fill(-1);
            int layer = 0;
            seen[start] = layer;
            while (queue.Count > 0)
            {
                int c = queue.Count;
                for (int _ = 0; _ < c; _ += 1)
                {
                    int curr = queue.Dequeue();
                    foreach (var next in adj[curr])
                    {
                        if (seen[next] == -1)
                        {
                            seen[next] = 1 + layer;
                            queue.Enqueue(next);
                        }
                        else if (seen[next] == layer)
                        {
                            return -1;
                        }
                    }
                }
                layer += 1;
            }
            return layer;
        }
    }
}

class DSU
{
    public int[] Parent { get; init; }
    public int[] Rank { get; init; }

    public DSU(int n)
    {
        Parent = [.. Enumerable.Range(0, n)]; // !!
        Rank = new int[n];
    }

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
        else { Rank[rx] += 1; Parent[ry] = rx; }
    }
}
