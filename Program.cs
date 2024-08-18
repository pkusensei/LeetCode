using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var node = TreeNode.Make([1, 2, 3, null, 5, null, 4]);
    var s = RightSideView(node);
    var a = "[1,3,4]";
    Debug.Assert(s.Print() == a, $"Output: {s.Print()}\nExpect: {a}");
}

void Test2()
{
    var node = TreeNode.Make([1, null, 3]);
    var s = RightSideView(node);
    var a = "[1,3]";
    Debug.Assert(s.Print() == a, $"Output: {s.Print()}\nExpect: {a}");
}

IList<int> RightSideView(TreeNode root)
{
    List<int> res = [];
    if (root is null) { return res; }

    var queue = new Queue<(TreeNode, int)>();
    var prev = (root, 0);
    queue.Enqueue(prev);
    while (queue.TryDequeue(out var item))
    {
        (var node, var level) = item;
        if (prev.Item2 != level)
        {
            res.Add(prev.Item1.val);
        }
        if (node.left is not null)
        {
            queue.Enqueue((node.left, level + 1));
        }
        if (node.right is not null)
        {
            queue.Enqueue((node.right, level + 1));
        }
        prev = item;
    }
    res.Add(prev.Item1.val);
    return res;
}