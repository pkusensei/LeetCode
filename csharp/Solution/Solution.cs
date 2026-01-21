using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumSimilarGroups(string[] strs)
    {
        int n = strs.Length;
        FrozenDictionary<string, int> dict
            = strs.Select((s, i) => (s, i)).ToFrozenDictionary(v => v.s, v => v.i);
        DSU dsu = new(n);
        for (int idx = 0; idx < n; idx++)
        {
            if (dsu.GetSize(idx) > 1) { continue; }
            Queue<int> queue = new();
            queue.Enqueue(idx);
            bool[] seen = new bool[n];
            seen[idx] = true;
            while (queue.TryDequeue(out var si))
            {
                foreach (var item in Next(strs[si]))
                {
                    if (dict.TryGetValue(item, out var next) && !seen[next])
                    {
                        seen[next] = true;
                        dsu.Union(idx, next);
                        queue.Enqueue(next);
                    }
                }
            }
        }
        return Enumerable.Range(0, n).Select(i => dsu.Find(i)).Distinct().Count();

        static IEnumerable<string> Next(string s)
        {
            int n = s.Length;
            for (int i1 = 0; i1 < n; i1++)
            {
                for (int i2 = 0; i2 < n; i2++)
                {
                    if (s[i1] != s[i2])
                    {
                        char[] arr = s.ToArray();
                        (arr[i1], arr[i2]) = (arr[i2], arr[i1]);
                        yield return new(arr);
                    }
                }
            }
        }
    }
}

struct DSU
{
    public DSU(int n)
    {
        Parent = [.. Enumerable.Range(0, n)];
        Size = [.. Enumerable.Repeat(1, n)];
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
            Parent[rx] = ry;
            Size[ry] += Size[rx];
        }
        else
        {
            Parent[ry] = rx;
            Size[rx] += Size[ry];
        }
    }

    public int GetSize(int v) => Size[Find(v)];
}
