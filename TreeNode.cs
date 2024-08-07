using System.Text;

public class TreeNode
{
    public int val;
    public TreeNode left;
    public TreeNode right;
    public TreeNode(int val = 0, TreeNode left = null, TreeNode right = null)
    {
        this.val = val;
        this.left = left;
        this.right = right;
    }

    public override string ToString()
    {
        var sb = new StringBuilder();
        sb.Append('[');
        var nodes = InorderFlatten().ToList();
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
        sb.Remove(sb.Length - 1, 1);
        sb.Append(']');

        return sb.ToString();
    }

    public IEnumerable<TreeNode> InorderFlatten()
    {
        var queue = new Queue<TreeNode>();
        queue.Enqueue(this);
        while (queue.TryDequeue(out var node))
        {
            if (node is not null)
            {
                yield return node;
                queue.Enqueue(node.left);
                queue.Enqueue(node.right);
            }
            else
            {
                yield return null;
            }
        }
    }

    public static TreeNode Make(IList<int?> values)
    {
        if (values.Count == 0 || !values[0].HasValue) { return null; }

        TreeNode root = new(values[0].Value);
        var queue = new Queue<TreeNode>();
        queue.Enqueue(root);
        var i = 1;
        while (queue.TryDequeue(out var curr) && i < values.Count)
        {
            if (i < values.Count && values[i].HasValue)
            {
                curr.left = new TreeNode(values[i].Value);
                queue.Enqueue(curr.left);
            }
            i += 1;
            if (i < values.Count && values[i].HasValue)
            {
                curr.right = new TreeNode(values[i].Value);
                queue.Enqueue(curr.right);
            }
            i += 1;
        }
        return root;
    }

    public static List<TreeNode> GenerateTrees(int n)
    {
        return Solve(1, n, []);

        static List<TreeNode> Solve(int start, int end, Dictionary<(int, int), List<TreeNode>> seen)
        {
            if (start > end)
            {
                return [null];
            }

            if (seen.TryGetValue((start, end), out var trees))
            {
                return trees;
            }

            List<TreeNode> res = [];
            for (int curr = start; curr <= end; curr++)
            {
                var leftTree = Solve(start, curr - 1, seen);
                var rightTree = Solve(curr + 1, end, seen);
                foreach (var left in leftTree)
                {
                    foreach (var right in rightTree)
                    {
                        TreeNode root = new(curr, left, right);
                        res.Add(root);
                    }
                }
            }
            seen.Add((start, end), res);
            return res;
        }
    }
}