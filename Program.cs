using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = TreeNode.Make([3, 9, 20, null, null, 15, 7]);
    var s = MinDepth(n);
    var a = 2;
    Debug.Assert(s == a, $"Output: {s}\nExpected:{a}");
}

void Test2()
{
    var n = TreeNode.Make([2, null, 3, null, 4, null, 5, null, 6]);
    var s = MinDepth(n);
    var a = 5;
    Debug.Assert(s == a, $"Output: {s}\nExpected:{a}");
}

int MinDepth(TreeNode root)
{
    if (root is null) { return 0; }

    var queue = new Queue<(TreeNode, int)>();
    queue.Enqueue((root, 1));
    while (queue.TryDequeue(out var item))
    {
        (var node, int depth) = item;
        if (node.left is null && node.right is null)
        {
            return depth;
        }
        if (node.left is not null)
        {
            queue.Enqueue((node.left, depth + 1));
        }
        if (node.right is not null)
        {
            queue.Enqueue((node.right, depth + 1));
        }

    }
    return 0;
}

