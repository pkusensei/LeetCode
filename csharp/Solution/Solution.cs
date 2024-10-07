// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<TreeNode> FindDuplicateSubtrees(TreeNode root)
    {
        var dict = new Dictionary<string, List<TreeNode>>();
        Collect(root, dict);
        return dict.Values.Where(v => v.Count > 1).Select(v => v.First()).ToList();
    }

    static void Collect(TreeNode node, IDictionary<string, List<TreeNode>> dict)
    {
        if (node is null) { return; }
        var s = Print(node);
        if (dict.TryGetValue(s, out var v)) { v.Add(node); }
        else { dict.Add(s, [node]); }
        Collect(node.left, dict);
        Collect(node.right, dict);
    }

    static string Print(TreeNode node)
    {
        var sb = new System.Text.StringBuilder();
        sb.Append('[');
        var nodes = Flatten(node).ToList();
        while (nodes.Last() is null)
        {
            nodes.RemoveAt(nodes.Count - 1);
        }

        foreach (var item in nodes)
        {
            if (item is not null)
            {
                sb.AppendFormat($"{item.val},");
            }
            else
            {
                sb.Append("null,");
            }
        }
        sb.Replace(',', ']', sb.Length - 1, 1);

        return sb.ToString();
    }

    static IEnumerable<TreeNode> Flatten(TreeNode n)
    {
        var queue = new Queue<TreeNode>();
        queue.Enqueue(n);
        while (queue.TryDequeue(out var node))
        {
            if (node is not null)
            {
                yield return node;
                queue.Enqueue(node.left);
                queue.Enqueue(node.right);
            }
            else { yield return null; }
        }
    }
}
