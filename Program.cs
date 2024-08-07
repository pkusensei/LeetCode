using System.Diagnostics;

// Test1();
Test2();
Test3();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    TreeNode n = TreeNode.Make([3, 9, 20, null, null, 15, 7]);
    var s = LevelOrder(n);
    var a = "[[3],[9,20],[15,7]]";
    Debug.Assert(s.ToString() == a, $"Expect: {s}\nOutput: {a}");
}

void Test2()
{
    TreeNode n = TreeNode.Make([1]);
    var s = LevelOrder(n);
    var a = "[1]";
    Debug.Assert(s.ToString() == a, $"Expect: {s}\nOutput: {a}");
}

void Test3()
{
    TreeNode n = TreeNode.Make([1, 2, null, 3, null, 4, null, 5]);
    var s = LevelOrder(n);
    var a = "[[1],[2],[3],[4],[5]]";
    Debug.Assert(s.ToString() == a, $"Expect: {s}\nOutput: {a}");
}

IList<IList<int>> LevelOrder(TreeNode root)
{
    var res = new List<IList<int>>();
    if (root is null) { return res; }

    var curr_level = 0;
    var curr = new List<int>();
    foreach (var (node, level) in InorderFlatten(root))
    {
        if (level == curr_level)
        {
            if (node is not null) { curr.Add(node.val); }
        }
        else
        {
            curr_level = level;
            res.Add(curr);
            curr = [];
            if (node is not null) { curr.Add(node.val); }
        }
    }
    if (curr.Count > 0) { res.Add(curr); }
    return res;
}

IEnumerable<(TreeNode node, int level)> InorderFlatten(TreeNode root)
{
    var queue = new Queue<(TreeNode, int)>();
    queue.Enqueue((root, 0));
    while (queue.TryDequeue(out var item))
    {
        var (node, level) = item;
        if (node is not null)
        {
            yield return item;
            queue.Enqueue((node.left, level + 1));
            queue.Enqueue((node.right, level + 1));
        }
        else
        {
            yield return item;
        }
    }
}