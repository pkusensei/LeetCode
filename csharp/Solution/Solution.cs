using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode DeleteNode(TreeNode root, int key)
    {
        if (root is null) { return null; }
        if (root.val < key) { root.right = DeleteNode(root.right, key); }
        else if (root.val > key) { root.left = DeleteNode(root.left, key); }
        else
        {
            switch ((root.left is null, root.right is null))
            {
                case (true, true): return null;
                case (true, false): return root.right;
                case (false, true): return root.left;
                default:
                    root.val = InorderSuccessor(ref root.right, root.right);
                    root.right = DeleteNode(root.right, root.val);
                    break;
            }
        }
        return root;
    }

    static int InorderSuccessor(ref TreeNode pos, TreeNode node)
    {
        if (node.left is not null) { return InorderSuccessor(ref node.left, node.left); }
        pos = node.right;
        return node.val; // node is dropped
    }
}