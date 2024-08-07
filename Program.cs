using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = LevelOrderBottom(TreeNode.Make([3, 9, 20, null, null, 15, 7]));
    var a = "[[15,7],[9,20],[3]]";
    Debug.Assert(n.ToString() == a, $"Expect: {n}\nOutput: {a}");
}

void Test2()
{
    var n = LevelOrderBottom(TreeNode.Make([1]));
    var a = "[[1]]";
    Debug.Assert(n.ToString() == a, $"Expect: {n}\nOutput: {a}");
}

IList<IList<int>> LevelOrderBottom(TreeNode root)
{
    var res = new List<IList<int>>();
    if (root is null) { return res; }

    var curr_level = 0;
    var curr = new List<int>();
    foreach (var (node, level) in PreorderFlatten(root))
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
    res.Reverse();
    return res;
}

IEnumerable<(TreeNode node, int level)> PreorderFlatten(TreeNode root)
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
