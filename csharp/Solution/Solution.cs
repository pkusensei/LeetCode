using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode BalanceBST(TreeNode root)
    {
        List<int> vals = [];
        Inorder(root);
        return Construct([.. vals]);

        void Inorder(TreeNode node)
        {
            if (node is null) { return; }
            Inorder(node.left);
            vals.Add(node.val);
            Inorder(node.right);
        }

        static TreeNode Construct(int[] vals)
        {
            if (vals.Length == 0) { return null; }
            if (vals.Length == 1) { return new(vals[0]); }
            var node = vals[vals.Length / 2];
            var left = Construct(vals[..(vals.Length / 2)]);
            var right = Construct(vals[(1 + vals.Length / 2)..]);
            return new(node, left, right);
        }
    }

    public TreeNode WithDSW(TreeNode root)
    {
        if (root is null) { return null; }
        TreeNode vinehead = new(0, null, root);
        var curr = vinehead;
        while (curr.right is not null)
        {
            if (curr.right.left is not null) { RotateRight(curr, curr.right); }
            else { curr = curr.right; }
        }
        var count = 0;
        curr = vinehead.right;
        while (curr is not null)
        {
            curr = curr.right;
            count += 1;
        }
        var depth = (int)Math.Pow(2, Math.Floor(Math.Log2(1 + count))) - 1;
        MakeRotations(vinehead, count - depth);
        while (depth > 1)
        {
            depth /= 2;
            MakeRotations(vinehead, depth);
        }
        return vinehead.right;

        static void RotateRight(TreeNode parent, TreeNode node)
        {
            TreeNode temp = node.left;
            node.left = temp.right;
            temp.right = node;
            parent.right = temp;
        }

        static void RotateLeft(TreeNode parent, TreeNode node)
        {
            TreeNode temp = node.right;
            node.right = temp.left;
            temp.left = node;
            parent.right = temp;
        }

        static void MakeRotations(TreeNode vinehead, int count)
        {
            var curr = vinehead;
            for (int i = 0; i < count; i++)
            {
                var temp = curr.right;
                RotateLeft(curr, temp);
                curr = curr.right;
            }
        }
    }
}
