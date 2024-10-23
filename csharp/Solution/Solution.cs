using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode ReplaceValueInTree(TreeNode root)
    {
        // return Bfs2Pass(root);
        // return Dfs(root);
        return Bfs1Pass(root);
    }

    static TreeNode Bfs2Pass(TreeNode root)
    {
        if (root is null) { return null; }
        List<int> level_sums = [];
        var level = new List<TreeNode> { root };
        while (level.Count > 0)
        {
            level_sums.Add(level.Select(n => n.val).Sum());
            var next = level.SelectMany(n => new[] { n.left, n.right }).Where(n => n is not null).ToList();
            level = next;
        }
        level = [root];
        root.val = 0;
        var sum_idx = 1;
        while (level.Count > 0)
        {
            List<TreeNode> next = [];
            foreach (var node in level)
            {
                var sibling_sum = (node.left is TreeNode n1 ? n1.val : 0) + (node.right is TreeNode n2 ? n2.val : 0);
                if (node.left is TreeNode n3)
                {
                    n3.val = level_sums[sum_idx] - sibling_sum;
                    next.Add(n3);
                }
                if (node.right is TreeNode n4)
                {
                    n4.val = level_sums[sum_idx] - sibling_sum;
                    next.Add(n4);
                }
            }
            level = next;
            sum_idx += 1;
        }
        return root;
    }

    static TreeNode Dfs(TreeNode root)
    {
        var sums = new int[100_000];
        LevelSum(root, 0);
        Replace(root, 0, 0);
        return root;

        void LevelSum(TreeNode node, int level)
        {
            if (node is null) { return; }
            sums[level] += node.val;
            LevelSum(node.left, 1 + level);
            LevelSum(node.right, 1 + level);
        }
        void Replace(TreeNode node, int level, int sibling_sum)
        {
            if (node is null) { return; }
            var left = node.left is TreeNode n1 ? n1.val : 0;
            var right = node.right is TreeNode n2 ? n2.val : 0;
            if (level == 0 || level == 1) { node.val = 0; }
            else { node.val = sums[level] - node.val - sibling_sum; }
            Replace(node.left, 1 + level, right);
            Replace(node.right, 1 + level, left);
        }
    }

    static TreeNode Bfs1Pass(TreeNode root)
    {
        if (root is null) { return null; }
        List<TreeNode> level = [root];
        // For root level, level sum is straightforward
        var prev_sum = root.val;
        while (level.Count > 0)
        {
            var curr_sum = 0;
            List<TreeNode> next = [];
            foreach (var node in level)
            {
                // For any other node, res = level_sum - sibling - self.val
                node.val = prev_sum - node.val;
                // On each level, find sum of next level first 
                // and record it in curr_sum, later prev_sum
                // Update each child node with sibling_sum
                // i.e left.val = left.val + right.val
                // Thus when the child node is popped, its final result should be
                // level_sum - self.val
                var sibling_sum = (node.left is TreeNode n1 ? n1.val : 0) + (node.right is TreeNode n2 ? n2.val : 0);
                if (node.left is TreeNode n3)
                {
                    curr_sum += n3.val;
                    n3.val = sibling_sum;
                    next.Add(n3);
                }
                if (node.right is TreeNode n4)
                {
                    curr_sum += n4.val;
                    n4.val = sibling_sum;
                    next.Add(n4);
                }
            }
            level = next;
            prev_sum = curr_sum;
        }
        return root;
    }
}
