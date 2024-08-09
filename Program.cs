using System.Diagnostics;

Test1();
Test2();
Test3();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = TreeNode.Make([1, 2, 5, 3, 4, null, 6]);
    Flatten(n);
    var a = "[1,null,2,null,3,null,4,null,5,null,6]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}

void Test2()
{
    var n = TreeNode.Make([]);
    Flatten(n);
    var a = "[]";
    Debug.Assert(n is null, $"Output: {n}\nExpect: {a}");
}

void Test3()
{
    var n = TreeNode.Make([0]);
    Flatten(n);
    var a = "[0]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}

void Flatten(TreeNode root)
{
    root = Solve(root).head;
}

(TreeNode head, TreeNode tail) Solve(TreeNode root)
{
    if (root is null) { return (null, null); }

    (var head, var tail) = (root, root);
    (var left, var right) = (root.left, root.right);
    (root.left, root.right) = (null, null);
    (var lhead, var ltail) = Solve(left);
    (var rhead, var rtail) = Solve(right);
    if (lhead is not null)
    {
        root.right = lhead;
        tail = ltail;
    }
    if (rhead is not null)
    {
        tail.right = rhead;
        tail = rtail;
    }
    return (head, tail);
}