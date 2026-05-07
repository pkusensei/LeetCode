using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> VerticalTraversal(TreeNode root)
    {
        Dictionary<int, List<(int row, int val)>> dict = [];
        Dfs(root, 0, 0);
        List<(int col, List<int> vals)> res = [];
        foreach (var item in dict)
        {
            item.Value.Sort((a, b) => a.row == b.row ? a.val.CompareTo(b.val) : a.row.CompareTo(b.row));
            res.Add((item.Key, item.Value.Select(v => v.val).ToList()));
        }
        res.Sort((a, b) => a.col.CompareTo(b.col));
        return [.. res.Select(v => v.vals)];

        void Dfs(TreeNode node, int row, int col)
        {
            if (node is null) { return; }
            if (!dict.TryAdd(col, [(row, node.val)]))
            {
                dict[col].Add((row, node.val));
            }
            Dfs(node.left, 1 + row, col - 1);
            Dfs(node.right, 1 + row, col + 1);
        }
    }
}
