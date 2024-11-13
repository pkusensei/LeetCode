using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{

    public int MinCameraCover(TreeNode root)
    {
        int res = 0;
        HashSet<TreeNode> nodes = [null]; // avoid double counting at leaf
        Dfs(root, null);
        return res;

        void Dfs(TreeNode node, TreeNode parent)
        {
            if (node is null) { return; }
            Dfs(node.left, node);
            Dfs(node.right, node);
            // Child nodes have to be covered
            // This node have to be covered, along with its parent
            if (!nodes.Contains(node.left) || !nodes.Contains(node.right) ||
            (!nodes.Contains(node) && parent is null))
            {
                nodes.Add(node.left);
                nodes.Add(node.right);
                nodes.Add(node);
                nodes.Add(parent);
                res += 1;
            }
        }
    }

    public int BetterDfs(TreeNode root)
    {
        int res = 0;
        return (Dfs(root) == 0 ? 1 : 0) + res;
        int Dfs(TreeNode node)
        {
            // States
            // 0 => All children are covered, but not this one
            // 1 => node and its children are covered, but no camera at node
            // 2 => ndoe and its children are covered, with camera at node
            if (node is null) { return 1; }
            var left = Dfs(node.left);
            var right = Dfs(node.right);
            if (left == 0 || right == 0)
            {
                res += 1; // Add camera at node
                return 2;
            }
            else if (left == 2 || right == 2)
            {
                return 1; // node is covered
            }
            else { return 0; }
        }
    }

    public int WithDp(TreeNode root)
    {
        var res = Solve(root);
        return Math.Min(res[1], res[2]);

        int[] Solve(TreeNode node)
        {
            // States
            // 0 => All children are covered, but not this one
            // 1 => node and its children are covered, but no camera at node
            // 2 => ndoe and its children are covered, with camera at node
            if (node is null) { return [0, 0, 99999]; }
            var left = Solve(node.left);
            var right = Solve(node.right);
            var dp0 = left[1] + right[1];
            var dp1 = Math.Min(left[2] + right[1..].Min(), right[2] + left[1..].Min());
            var dp2 = 1 + left.Min() + right.Min();
            return [dp0, dp1, dp2];
        }
    }
}
