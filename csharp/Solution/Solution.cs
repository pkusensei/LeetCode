using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int CountPairs(TreeNode root, int distance)
    {
        var res = 0;
        Dfs(root);
        return res;

        Dictionary<int, int> Dfs(TreeNode node)
        {
            if (node is null) { return null; }
            var left = Dfs(node.left);
            var right = Dfs(node.right);
            if (left is null && right is null)
            {
                return new() { { 1, 1 } };
            }
            if (left is null && right is not null)
            {
                return right.Select(item => (1 + item.Key, item.Value)).ToDictionary();
            }
            if (left is not null && right is null)
            {
                return left.Select(item => (1 + item.Key, item.Value)).ToDictionary();
            }
            foreach (var (k1, v1) in left)
            {
                foreach (var (k2, v2) in right)
                {
                    if (k1 + k2 <= distance) { res += v1 * v2; }
                }
            }
            foreach (var (k2, v2) in right)
            {
                if (!left.TryAdd(k2, v2)) { left[k2] += v2; }
            }
            return left.Select(item => (1 + item.Key, item.Value)).ToDictionary();
        }
    }

}
