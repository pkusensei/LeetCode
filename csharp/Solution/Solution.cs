using System.Collections.Immutable;
using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int DeepestLeavesSum(TreeNode root)
    {
        var max_depth = 0;
        var res = 0;
        Dfs(root, 0);
        return res;

        void Dfs(TreeNode node, int depth)
        {
            if (node is null) { return; }
            if (depth == max_depth) { res += node.val; }
            if (depth > max_depth) { max_depth = depth; res = node.val; }
            Dfs(node.left, 1 + depth);
            Dfs(node.right, 1 + depth);
        }
    }
}