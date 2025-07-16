using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LongestConsecutive(int[] nums)
    {
        if (nums.Length < 1) { return 0; }
        Dictionary<int, int> dict = [];
        foreach (var num in nums)
        {
            dict.TryAdd(num, dict.Count);
        }
        DSU dsu = new(dict.Count);
        foreach (var (num, id) in dict)
        {
            if (dict.TryGetValue(num - 1, out var i)) { dsu.Union(id, i); }
            if (dict.TryGetValue(1 + num, out i)) { dsu.Union(id, i); }
        }
        return dsu.Size.Max();
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
        if (Size[rx] < Size[ry]) { (rx, ry) = (ry, rx); }
        Parent[ry] = rx;
        Size[rx] += Size[ry];
    }
}