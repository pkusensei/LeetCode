using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinSwapsCouples(int[] row)
    {
        int n = row.Length / 2;
        DSU dsu = new(n);
        int[] seats = [.. Enumerable.Repeat(-1, n)];
        for (int i = 0; i < row.Length; i++)
        {
            int group = row[i] / 2;
            if (seats[group] < 0) { seats[group] = i / 2; }
            else { dsu.Union(seats[group], i / 2); }
        }
        int res = 0;
        for (int i = 0; i < n; i++)
        {
            int root = dsu.Find(i);
            if (seats[root] >= 0)
            {
                res += dsu.Size[root] - 1;
                seats[root] = -1;
            }
        }
        return res;
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
            Parent[rx] = ry;
            Size[ry] += Size[rx];
        }
        else
        {
            Parent[ry] = rx;
            Size[rx] += Size[ry];
        }
    }
}