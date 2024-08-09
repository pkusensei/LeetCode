using System.Diagnostics;

Test1();
Test2();
Test3();
Test4();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = Node.Make([1, 2, 3, 4, 5, null, 7]);
    var s = Connect(n);
    var a = "[1,#,2,3,#,4,5,7,#]";
    Debug.Assert(s.ToString() == a, $"Output: {n}\nExpect: {a}");
}

void Test2()
{
    var n = Node.Make([]);
    var s = Connect(n);
    var a = "[]";
    Debug.Assert(s is null, $"Output: {n}\nExpect: {a}");
}

void Test3()
{
    var n = Node.Make([-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    var s = Connect(n);
    var a = "[-1,#,0,1,#,2,3,4,5,#,6,7,8,9,10,11,12,13,#]";
    Debug.Assert(s.ToString() == a, $"Output: {n}\nExpect: {a}");
}

void Test4()
{
    var n = Node.Make([1, -4, -3, 5, -2, -2, -5]);
    var s = Connect(n);
    var a = "[1,#,-4,-3,#,5,-2,-2,-5,#]";
    Debug.Assert(s.ToString() == a, $"Output: {n}\nExpect: {a}");
}

Node Connect(Node root)
{
    if (root is null) { return root; }
    (Node node, int level) curr = (null, -1);
    var queue = new Queue<(Node node, int level)>();
    queue.Enqueue((root, 0));
    while (queue.TryDequeue(out var item))
    {
        if (curr.level == item.level)
        {
            curr.node.next = item.node;
        }
        (var left, var right) = (item.node.left, item.node.right);
        curr = item;
        if (left is not null)
        {
            queue.Enqueue((left, curr.level + 1));
        }
        if (right is not null)
        {
            queue.Enqueue((right, curr.level + 1));
        }
    }
    return root;
}