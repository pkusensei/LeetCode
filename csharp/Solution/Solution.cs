using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<IList<int>> VerticalTraversal(TreeNode root)
    {
        SortedDictionary<int, List<(int row, int val)>> map = [];
        Queue<(TreeNode node, int row, int col)> queue = [];
        queue.Enqueue((root, 0, 0));
        while (queue.TryDequeue(out var item))
        {
            var (node, row, col) = item;
            if (node is not null)
            {
                if (map.TryGetValue(col, out var list))
                {
                    list.Add((row, node.val));
                }
                else
                {
                    map.Add(col, [(row, node.val)]);
                }
                queue.Enqueue((node.left, row + 1, col - 1));
                queue.Enqueue((node.right, row + 1, col + 1));
            }
        }
        List<IList<int>> res = [];
        foreach (var v in map.Values)
        {
            res.Add(v.OrderBy(n => n.row).ThenBy(n => n.val).Select(n => n.val).ToList());
        }
        return res;
    }
}
