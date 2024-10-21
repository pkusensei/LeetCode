using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MinDiffInBST(TreeNode root)
    {
        List<int> list = [];
        Inorder(root, list);
        return list.Skip(1).Zip(list).Select(p => p.First - p.Second).Min();
    }

    static void Inorder(TreeNode node, List<int> list)
    {
        if (node is null) { return; }
        Inorder(node.left, list);
        list.Add(node.val);
        Inorder(node.right, list);
    }
}
