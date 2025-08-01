using System.Text;

namespace Solution.Tree;

public abstract class TreeNodeBase<T> where T : TreeNodeBase<T>, new()
{
    public int val;
    public T left;
    public T right;

    public IEnumerable<T> LevelOrderFlatten()
    {
        Queue<T> queue = [];
        queue.Enqueue((T)this);
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

    public IEnumerable<T> InorderFlatten()
    {
        var stack = new Stack<T>();
        var curr = (T)this;
        Push(stack, curr);
        while (stack.TryPop(out var node))
        {
            if (node is not null)
            {
                yield return node;
                Push(stack, node.right);
            }
            else { yield return null; }
        }

        static void Push(Stack<T> stack, T node)
        {
            while (node is not null)
            {
                stack.Push(node);
                node = node.left;
            }
        }
    }

    public IEnumerable<T> PreorderFlatten()
    {
        var stack = new Stack<T>();
        stack.Push((T)this);
        while (stack.TryPop(out var node))
        {
            if (node is not null)
            {
                yield return node;
                stack.Push(node.right);
                stack.Push(node.left);
            }
            else { yield return null; }
        }
    }

    IEnumerable<T> PostorderFlatten()
    {
        var stack = new Stack<T>();
        var curr = (T)this;
        T last = null;
        while (curr is not null || stack.Count > 0)
        {
            if (curr is not null)
            {
                stack.Push(curr);
                curr = curr.left;
            }
            else
            {
                var peek = stack.Peek();
                if (peek.right is not null && last != peek.right)
                {
                    curr = peek.right;
                }
                else
                {
                    yield return peek;
                    last = stack.Pop();
                }
            }
        }
    }

    public static T Make(IList<int?> values)
    {
        if (values.FirstOrDefault() is null) { return null; }
        T root = new() { val = values[0].Value };
        Queue<T> queue = [];
        queue.Enqueue(root);
        int i = 1;
        while (queue.TryDequeue(out var curr) && i < values.Count)
        {
            if (values[i] is int v1)
            {
                curr.left = new() { val = v1 };
                queue.Enqueue(curr.left);
            }
            i += 1;
            if (i < values.Count && values[i] is int v2)
            {
                curr.right = new() { val = v2 };
                queue.Enqueue(curr.right);
            }
            i += 1;
        }
        return root;
    }

    public static T MakeInt(IList<int> values)
    {
        if (values.Count == 0) { return null; }
        T root = new() { val = values[0] };
        Queue<T> queue = [];
        queue.Enqueue(root);
        int i = 1;
        while (queue.TryDequeue(out var curr) && i < values.Count)
        {

            curr.left = new() { val = values[i] };
            queue.Enqueue(curr.left);
            i += 1;
            if (i < values.Count)
            {
                curr.right = new() { val = values[i] };
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
        var nodes = LevelOrderFlatten().ToList();
        while (nodes.Count > 0 && nodes[^1] is null)
        {
            nodes.RemoveAt(nodes.Count - 1);
        }
        foreach (var item in nodes)
        {
            if (item is null) { sb.Append("null,"); }
            else { sb.Append($"{item.val},"); }
        }
        sb.Replace(',', ']', sb.Length - 1, 1);
        return sb.ToString();
    }
}

public class TreeNode : TreeNodeBase<TreeNode>
{
    public TreeNode() { }
    public TreeNode(int _val = 0, TreeNode _left = null, TreeNode _right = null)
    {
        val = _val;
        left = _left;
        right = _right;
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

// public class Node : TreeNodeBase<Node>
// {
//     public Node next;
//     public Node() { }
//     public Node(int _val) { val = _val; }
//     public Node(int _val, Node _left, Node _right, Node _next)
//     {
//         val = _val;
//         left = _left;
//         right = _right;
//         next = _next;
//     }
// }

public class Node
{
    public int val;
    public IList<Node> children;

    public Node()
    {
        val = 0;
        children = [];
    }

    public Node(int _val)
    {
        val = _val;
        children = [];
    }

    public Node(int _val, List<Node> _children)
    {
        val = _val;
        children = _children;
    }

    public override string ToString()
    {
        if (children.Count == 0) { return "[[]]"; }

        var values = Flatten().OrderBy(p => p.Key).Select(p => p.Value).ToList();
        var sb = new StringBuilder();
        sb.Append('[');
        foreach (var item in values)
        {
            sb.Append('[');
            sb.AppendJoin(',', item);
            sb.Append("],");
        }
        sb.Replace(',', ']', sb.Length - 1, 1);
        return sb.ToString();
    }

    public IDictionary<int, HashSet<int>> Flatten()
    {
        var res = new Dictionary<int, HashSet<int>>();
        var queue = new Queue<Node>();
        var seen = new HashSet<int>();
        queue.Enqueue(this);
        while (queue.TryDequeue(out var curr))
        {
            if (!seen.Add(curr.val)) { continue; }

            foreach (var neighbor in curr.children)
            {
                if (res.TryGetValue(curr.val, out var lst))
                {
                    lst.Add(neighbor.val);
                }
                else
                {
                    res.Add(curr.val, [neighbor.val]);
                }
                if (res.TryGetValue(neighbor.val, out var lst2))
                {
                    lst2.Add(curr.val);
                }
                else
                {
                    res.Add(neighbor.val, [curr.val]);
                }
                queue.Enqueue(neighbor);
            }
        }
        return res;
    }

    // public static Node Make(IList<IList<int>> values)
    // {
    //     var dict = new Dictionary<int, Node>();
    //     for (int i = 0; i < values.Count; i++)
    //     {
    //         dict.Add(i + 1, new Node(i + 1));
    //     }
    //     foreach (var (idx, lst) in values.Select((v, i) => (i + 1, v)))
    //     {
    //         foreach (var neighbor in lst)
    //         {
    //             dict[idx].children.Add(dict[neighbor]);
    //         }
    //     }
    //     return dict.Count == 0 ? null : dict[1];
    // }
}

public class Codec
{
    // Encodes a tree to a single string.
    public static string Serialize(TreeNode root)
    {
        if (root is null) { return "[]"; }
        return root.ToString();
    }

    // Decodes your encoded data to tree.
    public static TreeNode Deserialize(string data) => TreeNode.Make(Parse(data));

    static int?[] Parse(string data) => [.. data.Trim(['[', ']']).Split(',')
                                                .Select(s => int.TryParse(s, out int v) ? v : (int?)null)];
}