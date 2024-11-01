using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool LeafSimilar(TreeNode root1, TreeNode root2)
    {
        (var n1, var n2) = (new List<int>(), new List<int>());
        Dfs(root1, n1);
        Dfs(root2, n2);
        return n1.SequenceEqual(n2);
    }

    static void Dfs(TreeNode node, List<int> nums)
    {
        if (node is null) { return; }
        Dfs(node.left, nums);
        Dfs(node.right, nums);
        if (node.left is null && node.right is null)
        {
            nums.Add(node.val);
        }
    }
}
