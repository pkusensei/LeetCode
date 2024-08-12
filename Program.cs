using System.Diagnostics;
using System.Text;

Test1();
Test2();

Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = Node.Make([[2, 4], [1, 3], [2, 4], [1, 3]]);
    var s = CloneGraph(n);
    var a = "[[2,4],[1,3],[2,4],[1,3]]";
    Debug.Assert(s.ToString() == a, $"Output: {s}\nExpect: {a}");
}

void Test2()
{
    var n = Node.Make([[]]);
    var s = CloneGraph(n);
    var a = "[[]]";
    Debug.Assert(s.ToString() == a, $"Output: {s}\nExpect: {a}");
}

void Test3()
{
    var n = Node.Make([]);
    var s = CloneGraph(n);
    var a = "[]";
    Debug.Assert(s is null, $"Output: {n}\nExpect: {a}");

}

Node CloneGraph(Node node)
{
    if (node is null) { return null; }

    var queue = new Queue<Node>();
    var dict = new Dictionary<int, (Node, List<int>)>();
    var seen = new HashSet<int>();

    queue.Enqueue(node);
    while (queue.TryDequeue(out var curr))
    {
        if (!seen.Add(curr.val)) { continue; }

        var nums = curr.neighbors.Select(n => n.val).ToList();
        dict.Add(curr.val, (new(curr.val), nums));
        foreach (var item in curr.neighbors)
        {
            queue.Enqueue(item);
        }
    }
    foreach (var (curr, values) in dict.Values)
    {
        curr.neighbors = values.Select(num => dict[num].Item1).ToList();
    }

    return dict[1].Item1;
}

public class Node
{
    public int val;
    public IList<Node> neighbors;

    public Node()
    {
        val = 0;
        neighbors = [];
    }

    public Node(int _val)
    {
        val = _val;
        neighbors = [];
    }

    public Node(int _val, List<Node> _neighbors)
    {
        val = _val;
        neighbors = _neighbors;
    }

    public override string ToString()
    {
        if (neighbors.Count == 0) { return "[[]]"; }

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

            foreach (var neighbor in curr.neighbors)
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

    public static Node Make(IList<IList<int>> values)
    {
        var dict = new Dictionary<int, Node>();
        for (int i = 0; i < values.Count; i++)
        {
            dict.Add(i + 1, new(i + 1));
        }
        foreach (var (idx, lst) in values.Select((v, i) => (i + 1, v)))
        {
            foreach (var neighbor in lst)
            {
                dict[idx].neighbors.Add(dict[neighbor]);
            }
        }
        return dict.Count == 0 ? null : dict[1];
    }
}
