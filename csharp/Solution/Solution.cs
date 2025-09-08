using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int ArrayNesting(int[] nums)
    {
        DSU dsu = new(nums.Length);
        for (int i = 0; i < nums.Length; i++)
        {
            dsu.Union(i, nums[i]);
        }
        return dsu.Size.Max();
    }
}

class DSU
{
    public DSU(int n)
    {
        Parent = [.. Enumerable.Range(0, n)];
        Size = new int[n];
        Array.Fill(Size, 1);
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
        if (Size[rx] >= Size[ry])
        {
            Parent[ry] = rx;
            Size[rx] += Size[ry];
        }
        else
        {
            Parent[rx] = ry;
            Size[ry] += Size[rx];
        }
    }
}