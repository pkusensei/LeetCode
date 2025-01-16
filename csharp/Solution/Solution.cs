using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int PseudoPalindromicPaths(TreeNode root)
    {
        return Dfs(root, 0);

        static int Dfs(TreeNode node, int mask)
        {
            if (node is null) { return 0; }
            mask ^= 1 << node.val;
            if (node.left is null && node.right is null)
            {
                return Count(mask) <= 1 ? 1 : 0;
            }
            return Dfs(node.left, mask) + Dfs(node.right, mask);
        }

        static int Count(int val)
        {
            var res = 0;
            while (val != 0)
            {
                res += 1;
                val &= val - 1;
            }
            return res;
        }
    }
}
