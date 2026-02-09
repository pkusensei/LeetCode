using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode BalanceBST(TreeNode root)
    {
        List<int> vals = [];
        Inorder(root);
        return Build(vals);

        void Inorder(TreeNode node)
        {
            if (node is null) { return; }
            Inorder(node.left);
            vals.Add(node.val);
            Inorder(node.right);
        }

        static TreeNode Build(List<int> vals)
        {
            if (vals.Count == 0) { return null; }
            int n = vals.Count;
            TreeNode node = new(vals[n / 2]);
            node.left = Build(vals[..(n / 2)]);
            node.right = Build(vals[(1 + n / 2)..]);
            return node;
        }
    }
}