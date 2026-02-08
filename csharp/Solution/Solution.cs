using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsBalanced(TreeNode root)
    {
        return Dfs(root).Item1;

        static (bool, int) Dfs(TreeNode node)
        {
            if (node is null) { return (true, 0); }
            var left = Dfs(node.left);
            var right = Dfs(node.right);
            if (left.Item1 && right.Item1)
            {
                int max = int.Max(left.Item2, right.Item2);
                if (int.Abs(left.Item2 - right.Item2) <= 1) { return (true, 1 + max); }
                else { return (false, 0); }
            }
            else
            {
                return (false, 0);
            }
        }
    }
}