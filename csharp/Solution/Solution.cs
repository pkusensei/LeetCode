using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int KthLargestPerfectSubtree(TreeNode root, int k)
    {
        List<int> sizes = [];
        Dfs(root);
        sizes.Sort((a, b) => b - a);
        return k <= sizes.Count ? sizes[k - 1] : -1;

        int? Dfs(TreeNode node)
        {
            if (node is null) { return 0; }
            var left = Dfs(node.left);
            var right = Dfs(node.right);
            if (left.HasValue && right.HasValue && left.Value == right.Value)
            {
                int val = 1 + 2 * left.Value;
                sizes.Add(val);
                return val;
            }
            else { return null; }
        }
    }
}
