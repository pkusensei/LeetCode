using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindBottomLeftValue(TreeNode root)
    {
        int max_depth = -1;
        int res = 0;
        Dfs(root, 0);
        return res;

        void Dfs(TreeNode node, int depth)
        {
            if (node is null) { return; }
            if (depth > max_depth)
            {
                max_depth = depth;
                res = node.val;
            }
            Dfs(node.left, 1 + depth);
            Dfs(node.right, 1 + depth);
        }
    }
}