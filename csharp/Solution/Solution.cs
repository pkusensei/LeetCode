using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    Dictionary<(TreeNode, int), int> memo = [];

    public int LongestZigZag(TreeNode root)
    {
        if (root is null) { return 0; }
        int a = Dfs(root, 0, memo);
        int b = Dfs(root, 1, memo);
        int c = LongestZigZag(root.left);
        int d = LongestZigZag(root.right);
        return Math.Max(a, Math.Max(b, Math.Max(c, d)));

        static int Dfs(TreeNode node, int dir, IDictionary<(TreeNode, int), int> memo)
        {
            if (node is null) { return -1; }
            if (memo.TryGetValue((node, dir), out var val)) { return val; }
            var res = 0;
            if (dir == 0) { res = 1 + Dfs(node.left, 1 - dir, memo); }
            else { res = 1 + Dfs(node.right, 1 - dir, memo); }
            memo.Add((node, dir), res);
            return res;
        }
    }
}