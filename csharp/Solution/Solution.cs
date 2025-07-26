using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Codec
{
    // Encodes a tree to a single string.
    public string serialize(TreeNode root)
    {
        if (root is null) { return "[]"; }
        StringBuilder sb = new();
        sb.Append('[');
        List<TreeNode> nodes = [.. LevelOrder(root)];
        while (nodes.Count > 0 && nodes[^1] is null)
        {
            nodes.RemoveAt(nodes.Count - 1);
        }
        foreach (var node in nodes)
        {
            if (node is null) { sb.Append("null,"); }
            else { sb.Append($"{node.val},"); }
        }
        sb.Replace(',', ']', sb.Length - 1, 1);
        return sb.ToString();

        static IEnumerable<TreeNode> LevelOrder(TreeNode root)
        {
            Queue<TreeNode> queue = [];
            queue.Enqueue(root);
            while (queue.TryDequeue(out var node))
            {
                yield return node;
                if (node is not null)
                {
                    queue.Enqueue(node.left);
                    queue.Enqueue(node.right);
                }
            }
        }
    }

    // Decodes your encoded data to tree.
    public TreeNode deserialize(string data)
    {
        int?[] vals = [.. data.Trim(['[', ']'])
                              .Split(',')
                              .Select(s => int.TryParse(s, out int v) ? v : (int?)null)];
        if (vals.FirstOrDefault() is null) { return null; }
        TreeNode root = new(vals[0].Value);
        Queue<TreeNode> queue = [];
        queue.Enqueue(root);
        int idx = 1;
        while (queue.TryDequeue(out var node) && idx < vals.Length)
        {
            if (vals[idx] is int v)
            {
                node.left = new(v);
                queue.Enqueue(node.left);
            }
            idx += 1;
            if (idx < vals.Length && vals[idx] is int v2)
            {
                node.right = new(v2);
                queue.Enqueue(node.right);
            }
            idx += 1;
        }
        return root;
    }
}