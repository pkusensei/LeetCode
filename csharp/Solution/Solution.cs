using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<string>> AccountsMerge(IList<IList<string>> accounts)
    {
        int n = accounts.Count;
        DSU dsu = new(n);
        for (int a = 0; a < n; a++)
        {
            for (int b = 1 + a; b < n; b++)
            {
                if (accounts[a].Skip(1).Intersect(accounts[b].Skip(1)).Any())
                {
                    dsu.Union(a, b);
                }
            }
        }
        Dictionary<int, HashSet<string>> roots = [];
        for (int i = 0; i < n; i++)
        {
            int root = dsu.Find(i);
            if (!roots.TryAdd(root, [.. accounts[i].Skip(1)]))
            {
                roots[root].UnionWith(accounts[i].Skip(1));
            }
        }
        List<IList<string>> res = new(roots.Count);
        foreach (var (k, v) in roots)
        {
            List<string> curr = [accounts[k][0]];
            curr.AddRange(v.Order(Comparer<string>.Create(string.CompareOrdinal)));
            res.Add(curr);
        }
        return res;
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

    public void Union(int x, int y)
    {
        int rx = Find(x);
        int ry = Find(y);
        if (rx == ry) { return; }
        if (Rank[rx] < Rank[ry]) { Parent[rx] = ry; }
        else if (Rank[rx] == Rank[ry])
        {
            Parent[ry] = rx;
            Rank[rx] += 1;
        }
        else { Parent[ry] = rx; }
    }
}