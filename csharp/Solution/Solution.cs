using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumOfUnplacedFruits(int[] fruits, int[] baskets)
    {
        SegTree tree = new(baskets);
        int res = 0;
        foreach (var f in fruits)
        {
            res += tree.Query(f) ? 0 : 1;
        }
        return res;
    }
}

internal class SegTree
{
    public SegTree(ReadOnlySpan<int> nums)
    {
        N = nums.Length;
        Tree = new int[4 * N];
        Build(1, 0, N - 1, nums);
    }

    int N { get; }
    int[] Tree { get; }

    private void Build(int node, int left, int right, ReadOnlySpan<int> nums)
    {
        if (left == right)
        {
            Tree[node] = nums[left];
            return;
        }
        int mid = left + (right - left) / 2;
        Build(2 * node, left, mid, nums);
        Build(2 * node + 1, 1 + mid, right, nums);
        Tree[node] = int.Max(Tree[2 * node], Tree[2 * node + 1]);
    }

    public void Update(int idx, int val) => Update(1, 0, N - 1, idx, val);

    private void Update(int node, int left, int right, int idx, int val)
    {
        if (left == right)
        {
            Tree[node] = val;
            return;
        }
        int mid = left + (right - left) / 2;
        if (idx <= mid) { Update(2 * node, left, mid, idx, val); }
        else { Update(2 * node + 1, 1 + mid, right, idx, val); }
        Tree[node] = int.Max(Tree[2 * node], Tree[2 * node + 1]);
    }

    public bool Query(int val) => Query(1, 0, N - 1, val);

    private bool Query(int node, int left, int right, int val)
    {
        if (Tree[node] < val) { return false; }
        if (left == right)
        {
            if (Tree[node] >= val)
            {
                Tree[node] = 0;
                return true;
            }
            return false;
        }
        int mid = left + (right - left) / 2;
        bool res = Query(2 * node, left, mid, val) || Query(2 * node + 1, 1 + mid, right, val);
        Tree[node] = int.Max(Tree[2 * node], Tree[2 * node + 1]);
        return res;
    }
}
