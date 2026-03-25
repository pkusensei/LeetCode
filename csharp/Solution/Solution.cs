using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int RemoveStones(int[][] stones)
    {
        int n = stones.Length;
        int[] x_i = [.. Enumerable.Repeat(10_001, 10_001)];
        int[] y_i = [.. Enumerable.Repeat(10_001, 10_001)];
        DSU dsu = new(n);
        for (int i = 0; i < n; i++)
        {
            int x = stones[i][0];
            int y = stones[i][1];
            x_i[x] = int.Min(i, x_i[x]);
            dsu.Union(x_i[x], i);
            y_i[y] = int.Min(i, y_i[y]);
            dsu.Union(y_i[y], i);
        }
        int[] arr = new int[n];
        for (int i = 0; i < n; i++)
        {
            int root = dsu.Find(i);
            arr[root] = dsu.Size[root];
        }
        return arr.Select(v => int.Max(v - 1, 0)).Sum();
    }
}

readonly struct DSU
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