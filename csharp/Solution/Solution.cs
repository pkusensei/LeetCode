using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode DeleteNode(TreeNode root, int key)
    {
        if (root is null) { return null; }
        if (key < root.val) { root.left = DeleteNode(root.left, key); }
        else if (key > root.val) { root.right = DeleteNode(root.right, key); }
        else
        {
            switch ((root.left is null, root.right is null))
            {
                case (true, true): return null;
                case (true, false): return root.right;
                case (false, true): return root.left;
                default:
                    int val = InorderSuccessor(root.right);
                    root.val = val;
                    root.right = DeleteNode(root.right, val);
                    break;
            }
        }
        return root;

        static int InorderSuccessor(TreeNode node)
        {
            while (node.left is not null)
            {
                node = node.left;
            }
            return node.val;
        }
    }
}