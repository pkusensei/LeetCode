using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode CreateBinaryTree(int[][] descriptions)
    {
        Dictionary<int, TreeNode> dict = [];
        HashSet<int> children = [];
        foreach (var item in descriptions)
        {
            int parent = item[0];
            int child = item[1];
            bool is_left = item[2] == 1;
            if (!dict.TryGetValue(parent, out TreeNode parent_node))
            {
                parent_node = new(parent);
            }
            if (!dict.TryGetValue(child, out var child_node))
            {
                child_node = new(child);
            }
            if (is_left) { parent_node.left = child_node; }
            else { parent_node.right = child_node; }
            dict.TryAdd(parent, parent_node);
            dict.TryAdd(child, child_node);
            children.Add(child);
        }
        int root = dict.Keys.Except(children).Single();
        return dict[root];
    }
}

