using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode IncreasingBST(TreeNode root)
    {
        return Dfs(root).h;

        static (TreeNode h, TreeNode t) Dfs(TreeNode node)
        {
            if (node is null) { return (null, null); }
            var a = Dfs(node.left);
            var b = Dfs(node.right);
            if (a.h is null)
            {
                TreeNode head = node;
                node.right = b.h;
                TreeNode tail = b.t is null ? node : b.t;
                return (head, tail);
            }
            else
            {
                TreeNode head = a.h;
                a.t.right = node;
                node.left = null;
                node.right = b.h;
                TreeNode tail = b.t is null ? node : b.t;
                return (head, tail);
            }
        }
    }
}