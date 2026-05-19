using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode InsertIntoMaxTree(TreeNode root, int val)
    {
        TreeNode dummy = new(-1, null, root);
        dummy.right = Dfs(root, val);
        return dummy.right;

        static TreeNode Dfs(TreeNode node, int val)
        {
            if (node is null) { return new(val); }
            if (node.val < val)
            {
                TreeNode temp = new(val);
                temp.left = node;
                return temp;
            }
            else
            {
                node.right = Dfs(node.right, val);
                return node;
            }
        }
    }
}

