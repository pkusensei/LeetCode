using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int[] TreeQueries(TreeNode root, int[] queries)
    {
        // For any node, the height after removing its subtree 
        // is the height of the tree before reaching that node
        var max_heights = new int[100_001];
        var curr = 0;
        LeftRight(root, 0);
        curr = 0;
        RightLeft(root, 0);
        return queries.Select(n => max_heights[n]).ToArray();

        void LeftRight(TreeNode node, int height)
        {
            if (node is null) { return; }
            max_heights[node.val] = curr;
            curr = Math.Max(height, curr);
            LeftRight(node.left, 1 + height);
            LeftRight(node.right, 1 + height);
        }
        void RightLeft(TreeNode node, int height)
        {
            if (node is null) { return; }
            max_heights[node.val] = Math.Max(curr, max_heights[node.val]);
            curr = Math.Max(height, curr);
            RightLeft(node.right, 1 + height);
            RightLeft(node.left, 1 + height);
        }
    }

    public int[] SinglePass(TreeNode root, int[] queries)
    {
        var results = new Dictionary<int, int>();
        var heights = new Dictionary<TreeNode, int>();
        Dfs(root, 0, 0);
        return queries.Select(n => results[n]).ToArray();

        int Height(TreeNode node)
        {
            if (node is null) { return -1; }
            if (heights.TryGetValue(node, out var v)) { return v; }
            var h = 1 + Math.Max(Height(node.left), Height(node.right));
            heights.Add(node, h);
            return h;
        }
        void Dfs(TreeNode node, int depth, int max_val)
        {
            if (node is null) { return; }
            results.Add(node.val, max_val);
            Dfs(node.left, 1 + depth, Math.Max(max_val, 1 + depth + Height(node.right)));
            Dfs(node.right, 1 + depth, Math.Max(max_val, 1 + depth + Height(node.left)));
        }
    }

    public int[] EulerianTour(TreeNode root, int[] queries)
    {
        List<int> euler_tour = [];
        Dictionary<int, int> heights = [];
        Dictionary<int, int> first_occur = [];
        Dictionary<int, int> last_occur = [];
        Dfs(root, 0);

        var count = euler_tour.Count;
        var left_max = new int[count];
        var right_max = new int[count];
        left_max[0] = right_max[count - 1] = heights[root.val];
        for (int i = 1; i < count; i++)
        {
            left_max[i] = Math.Max(left_max[i - 1], heights[euler_tour[i]]);
        }
        for (int i = count - 2; i >= 0; i -= 1)
        {
            right_max[i] = Math.Max(right_max[i + 1], heights[euler_tour[i]]);
        }

        return queries.Select(query =>
        {
            var left = first_occur[query] > 0 ? left_max[first_occur[query] - 1] : 0;
            var right = last_occur[query] > 0 ? right_max[last_occur[query] + 1] : 0;
            return Math.Max(left, right);
        }).ToArray();

        void Dfs(TreeNode node, int height)
        {
            if (node is null) { return; }
            heights.Add(node.val, height);
            first_occur.Add(node.val, euler_tour.Count);
            euler_tour.Add(node.val);

            Dfs(node.left, 1 + height);
            Dfs(node.right, 1 + height);

            last_occur.Add(node.val, euler_tour.Count);
            euler_tour.Add(node.val);
        }
    }

    public int[] TwoLargestCousins(TreeNode root, int[] queries)
    {
        Dictionary<int, int> node_depths = [];
        Dictionary<int, int> subtree_heights = [];
        Dictionary<int, int> first_max = [];
        Dictionary<int, int> second_max = [];
        Dfs(root, 0);
        return queries.Select(query =>
        {
            var level = node_depths[query];
            if (subtree_heights[query] == first_max[level])
            {
                return level + second_max.GetValueOrDefault(level, 0) - 1;
            }
            else
            {
                return level + first_max.GetValueOrDefault(level, 0) - 1;
            }
        }).ToArray();

        int Dfs(TreeNode node, int level)
        {
            if (node is null) { return 0; }

            node_depths.Add(node.val, level);
            var left = Dfs(node.left, 1 + level);
            var right = Dfs(node.right, 1 + level);
            var curr = 1 + Math.Max(left, right);
            subtree_heights.Add(node.val, curr);
            var curr_first_max = first_max.GetValueOrDefault(level, 0);
            if (curr > curr_first_max)
            {
                second_max[level] = curr_first_max;
                first_max[level] = curr;
            }
            else if (curr > second_max.GetValueOrDefault(level, 0))
            {
                second_max[level] = curr;
            }
            return curr;
        }
    }
}
