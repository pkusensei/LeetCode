using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> DistanceK(TreeNode root, TreeNode target, int k)
    {
        TreeNode[] parents = new TreeNode[501];
        FindParent(root, null);
        List<int> res = [];
        bool[] seen = new bool[501];
        Dfs(target, k);
        return res;

        void Dfs(TreeNode node, int k)
        {
            if (node is null || seen[node.val]) { return; }
            seen[node.val] = true;
            if (k == 0) { res.Add(node.val); }
            Dfs(node.left, k - 1);
            Dfs(node.right, k - 1);
            if (parents[node.val] is TreeNode n) { Dfs(n, k - 1); }
        }

        void FindParent(TreeNode node, TreeNode parent)
        {
            if (node is null) { return; }
            parents[node.val] = parent;
            FindParent(node.left, node);
            FindParent(node.right, node);
        }
    }
}