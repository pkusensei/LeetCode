using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindSecondMinimumValue(TreeNode root)
    {
        long min1 = long.MaxValue;
        long min2 = long.MaxValue;
        Dfs(root);
        return min2 > int.MaxValue ? -1 : (int)min2;

        void Dfs(TreeNode node)
        {
            if (node is null || node.val >= min2) { return; }
            if (node.val < min1)
            {
                min2 = min1;
                min1 = node.val;
            }
            else if (min1 < node.val)
            {
                min2 = node.val;
            }
            Dfs(node.left);
            Dfs(node.right);
        }
    }
}