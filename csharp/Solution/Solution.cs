using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool IsCousins(TreeNode root, int x, int y)
    {
        List<(int depth, int parent)> nodes = new(2);
        Dfs(root, -1, 0);
        return nodes[0].depth == nodes[1].depth && nodes[0].parent != nodes[1].parent;

        void Dfs(TreeNode node, int parent, int depth)
        {
            if (node is null) { return; }
            if (node.val == x || node.val == y)
            {
                nodes.Add((depth, parent));
                if (nodes.Count == 2) { return; }
            }
            Dfs(node.left, node.val, 1 + depth);
            Dfs(node.right, node.val, 1 + depth);
        }
    }
}
