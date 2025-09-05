using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int GetMinimumDifference(TreeNode root)
    {
        int res = 100_001;
        Dfs(root, -100_001, 200_001);
        return res;

        void Dfs(TreeNode node, int min, int max)
        {
            if (node is null) { return; }
            int val = node.val;
            res = int.Min(res, int.Min(val - min, max - val));
            Dfs(node.left, min, val);
            Dfs(node.right, val, max);
        }
    }

    public int Inorder(TreeNode root)
    {
        int res = 100_001;
        int prev = -100_001;
        Dfs(root);
        return res;

        void Dfs(TreeNode node)
        {
            if (node is null) { return; }
            Dfs(node.left);
            res = int.Min(res, node.val - prev);
            prev = node.val;
            Dfs(node.right);
        }
    }
}
