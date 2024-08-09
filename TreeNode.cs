using System.Text;

public abstract class NodeBase<T> where T : NodeBase<T>, new()
{
    public int val;
    public T left;
    public T right;

    public IEnumerable<T> PreorderFlatten()
    {
        var queue = new Queue<T>();
        queue.Enqueue((T)this);
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

    public static T Make(IList<int?> values)
    {
        if (values.Count == 0 || !values[0].HasValue) { return null; }

        T root = new() { val = values[0].Value };
        var queue = new Queue<T>();
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

    public override string ToString()
    {
        var sb = new StringBuilder();
        sb.Append('[');
        var nodes = PreorderFlatten().ToList();
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
}

public class TreeNode : NodeBase<TreeNode>
{
    public TreeNode() { }
    public TreeNode(int val = 0, TreeNode left = null, TreeNode right = null)
    {
        this.val = val;
        this.left = left;
        this.right = right;
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

public class Node : NodeBase<Node>
{
    public Node next;
    public Node() { }
    public Node(int _val) { val = _val; }
    public Node(int _val, Node _left, Node _right, Node _next)
    {
        val = _val;
        left = _left;
        right = _right;
        next = _next;
    }
}