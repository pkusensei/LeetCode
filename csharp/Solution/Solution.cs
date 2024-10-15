// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<IList<string>> AccountsMerge(IList<IList<string>> accounts)
    {
        DSU dsu = new(accounts.Count);

        var groups = new Dictionary<string, int>();
        foreach (var (idx, acc) in accounts.Select((acc, i) => (i, acc)))
        {
            var size = acc.Count;
            foreach (var em in acc.Skip(1))
            {
                if (groups.TryGetValue(em, out var i))
                {
                    dsu.UnionBySize(idx, i);
                }
                else
                {
                    groups.Add(em, idx);
                }
            }
        }

        var components = new Dictionary<int, List<string>>();
        foreach (var (em, group) in groups)
        {
            var rep = dsu.FindRepr(group);
            if (!components.TryGetValue(rep, out List<string> value))
            {
                value = ([]);
                components.Add(rep, value);
            }
            value.Add(em);
        }

        List<IList<string>> res = [];
        foreach (var (group, comp) in components)
        {
            comp.Sort(StringComparer.Ordinal);
            comp.Insert(0, accounts[group][0]);
            res.Add(comp);
        }
        return res;
    }
}

class DSU
{
    public int[] Reps { get; private set; }
    public int[] Size { get; private set; }
    public DSU(int n)
    {
        Reps = Enumerable.Range(0, n).ToArray();
        Size = Reps[..];
    }

    public int FindRepr(int x)
    {
        if (x == Reps[x]) { return x; }
        Reps[x] = FindRepr(Reps[x]); // compression
        return Reps[x];
    }

    public void UnionBySize(int a, int b)
    {
        var (rep_a, rep_b) = (FindRepr(a), FindRepr(b));
        if (rep_a == rep_b) { return; }

        if (Size[rep_a] >= Size[rep_b])
        {
            Size[rep_a] += Size[rep_b];
            Reps[rep_b] = Reps[rep_a];
        }
        else
        {
            Size[rep_b] += Size[rep_a];
            Reps[rep_a] = Reps[rep_b];
        }
    }
}