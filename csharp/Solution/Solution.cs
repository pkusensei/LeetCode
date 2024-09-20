using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Codec
{
    // Encodes a tree to a single string.
    public string Serialize(TreeNode root)
    {
        if (root is null) { return "[]"; }
        var sb = new System.Text.StringBuilder();
        sb.Append('[');
        var nodes = LevelOrderFlatten(root).ToList();
        while (nodes.Last() is null)
        {
            nodes.RemoveAt(nodes.Count - 1);
        }
        foreach (var item in nodes)
        {
            if (item is not null) { sb.AppendFormat($"{item.val},"); }
            else { sb.Append("null,"); }
        }
        sb.Replace(',', ']', sb.Length - 1, 1);
        return sb.ToString();
    }

    public IEnumerable<TreeNode> LevelOrderFlatten(TreeNode root)
    {
        var queue = new Queue<TreeNode>();
        queue.Enqueue(root);
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

    // Decodes your encoded data to tree.
    public TreeNode Deserialize(string data)
    {
        var values = Parse(data);
        if (values.Count == 0 || !values[0].HasValue) { return null; }

        TreeNode root = new(values[0].Value);
        var queue = new Queue<TreeNode>();
        queue.Enqueue(root);
        var i = 1;
        while (queue.TryDequeue(out var curr) && i < values.Count)
        {
            if (i < values.Count && values[i].HasValue)
            {
                curr.left = new() { val = values[i].Value };
                queue.Enqueue(curr.left);
            }
            i += 1;
            if (i < values.Count && values[i].HasValue)
            {
                curr.right = new() { val = values[i].Value };
                queue.Enqueue(curr.right);
            }
            i += 1;
        }
        return root;
    }

    static List<int?> Parse(string data) =>
        data.Trim(['[', ']']).Split(',').Select(s =>
        {
            if (int.TryParse(s, out int v)) { return v; }
            else { return (int?)null; }
        }).ToList();
}