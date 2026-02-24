using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int SumRootToLeaf(TreeNode root)
    {
        int res = 0;
        Dfs(root, 0);
        return res;

        void Dfs(TreeNode node, int curr)
        {
            if (node is null) { return; }
            curr = (curr << 1) | node.val;
            if (node.left is null && node.right is null) { res += curr; }
            Dfs(node.left, curr);
            Dfs(node.right, curr);
        }
    }
}