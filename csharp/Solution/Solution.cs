using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] FindRedundantDirectedConnection(int[][] edges)
    {
        int n = edges.Length;
        int[] inedge = [.. Enumerable.Repeat(-1, 1 + n)];
        int cand1 = -1;
        int cand2 = -1;
        for (int i = 0; i < n; i += 1)
        {
            if (inedge[edges[i][1]] != -1) // in-degree >= 2
            {
                cand1 = inedge[edges[i][1]];
                cand2 = i;
            }
            inedge[edges[i][1]] = i;
        }
        DSU dsu = new(1 + n);
        for (int i = 0; i < n; i++)
        {
            // ignore second in-edge; no union
            if (i == cand2) { continue; }
            if (!dsu.Union(edges[i][0], edges[i][1]))
            {
                // in-degree >= 2; first edge causes cycle
                if (cand2 >= 0) { return edges[cand1]; }
                // All in-degrees are 1; no parent conflict
                // this edge causing cycle
                else { return edges[i]; }
            }
        }
        return edges[cand2]; // in-degree >= 2; second edge is redundant
    }
}

readonly struct DSU
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