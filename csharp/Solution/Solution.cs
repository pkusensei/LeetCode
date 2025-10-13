using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LongestUnivaluePath(TreeNode root)
    {
        if (root is null) { return 0; }
        return int.Max(Dfs(root.left, root.val) + Dfs(root.right, root.val),
                    int.Max(LongestUnivaluePath(root.left), LongestUnivaluePath(root.right)));

        static int Dfs(TreeNode node, int val)
        {
            if (node is null || node.val != val) { return 0; }
            return 1 + int.Max(Dfs(node.left, val), Dfs(node.right, val));
        }
    }
}
