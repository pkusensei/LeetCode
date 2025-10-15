using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> FallingSquares(int[][] positions)
    {
        var compression = positions.SelectMany(p => new[] { p[0], p[0] + p[1] - 1 }).Distinct().Order().Select((num, i) => (num, i)).ToDictionary();
        int n = compression.Count;
        SegTree tree = new(n);
        List<int> res = new(n);
        foreach (var p in positions)
        {
            int left = compression[p[0]];
            int right = compression[p[0] + p[1] - 1];
            int prev = tree.Query(left, right);
            tree.Update(left, right, prev + p[1]);
            res.Add(tree.Query(0, n - 1));
        }
        return res;
    }
}

readonly struct SegTree
{
    public SegTree(int n)
    {
        Tree = new int[4 * n];
        N = n;
    }

    public int[] Tree { get; }
    public int N { get; }

    public int Query(int left, int right) => Query(1, 0, N - 1, left, right);

    private int Query(int node, int left, int right, int ql, int qr)
    {
        if (right < ql || qr < left) { return 0; }
        if (ql <= left && right <= qr) { return Tree[node]; }
        int mid = left + (right - left) / 2;
        return int.Max(Query(2 * node, left, mid, ql, qr),
                       Query(2 * node + 1, 1 + mid, right, ql, qr));
    }

    public void Update(int left, int right, int val) => Update(1, 0, N - 1, left, right, val);

    private void Update(int node, int left, int right, int ql, int qr, int val)
    {
        if (right < ql || qr < left) { return; }
        if (left == right)
        {
            Tree[node] = val;
            return;
        }
        int mid = left + (right - left) / 2;
        Update(2 * node, left, mid, ql, qr, val);
        Update(2 * node + 1, 1 + mid, right, ql, qr, val);
        Tree[node] = int.Max(Tree[2 * node], Tree[2 * node + 1]);
    }
}