// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<IList<string>> PrintTree(TreeNode root)
    {
        var m = Rows(root);
        int n = (int)Math.Pow(2, m) - 1;

        var res = new string[m][];
        for (int i = 0; i < m; i += 1)
        {
            res[i] = Enumerable.Range(0, n).Select(_ => "").ToArray();
        }
        Fill(res, root, m - 1, 0, (n - 1) / 2);
        return res;
    }

    static void Fill(string[][] res, TreeNode node, int height, int r, int c)
    {
        if (node is null) { return; }
        res[r][c] = node.val.ToString();
        Fill(res, node.left, height, r + 1, c - (int)Math.Pow(2, height - r - 1));
        Fill(res, node.right, height, r + 1, c + (int)Math.Pow(2, height - r - 1));
    }

    static int Rows(TreeNode node)
    {
        if (node is null) { return 0; }
        return 1 + Math.Max(Rows(node.left), Rows(node.right));
    }
}
