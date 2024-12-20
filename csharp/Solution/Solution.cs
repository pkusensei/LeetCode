using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode ReverseOddLevels(TreeNode root)
    {
        if (root is null || root.left is null) { return root; }
        List<TreeNode> odd = [root.left, root.right];
        while (odd.Count > 0)
        {
            (var left, var right) = (0, odd.Count - 1);
            while (left < right)
            {
                (odd[left].val, odd[right].val) = (odd[right].val, odd[left].val);
                left += 1; right -= 1;
            }
            odd = odd.SelectMany(n => new[] { n.left, n.right })
                     .Where(n => n is not null) // Next even level exists
                     .SelectMany(n => new[] { n.left, n.right })
                     .Where(n => n is not null) // Next odd level exists
                     .ToList();
        }
        return root;
    }

    static void Dfs(TreeNode n1, TreeNode n2, bool isOdd)
    {
        if (n1 is null || n2 is null) { return; }
        if (isOdd) { (n1.val, n2.val) = (n2.val, n1.val); }
        Dfs(n1.left, n2.right, !isOdd);
        Dfs(n1.right, n2.left, !isOdd);
    }
}