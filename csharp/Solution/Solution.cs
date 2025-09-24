using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool FindTarget(TreeNode root, int k)
    {
        return Dfs(root, root, k);

        static bool Dfs(TreeNode node, TreeNode root, int k)
        {
            if (node is null) { return false; }
            return Find(root, node, k - node.val)
                || Dfs(node.left, root, k) || Dfs(node.right, root, k);
        }

        static bool Find(TreeNode node, TreeNode used, int val)
        {
            if (node is null) { return false; }
            if (val == node.val && node != used) { return true; }
            if (val < node.val) { return Find(node.left, used, val); }
            return Find(node.right, used, val);
        }
    }
}