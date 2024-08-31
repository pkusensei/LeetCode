using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = NaryTree.TreeNode.Make([1, 2, 3, 4, 5, 6]);
    var s = CountNodes(n);
    var a = 6;
    Debug.Assert(s == a, $"Output: {s}\n Expected: {a}");
}

void Test2()
{
    var n = NaryTree.TreeNode.Make([1]);
    var s = CountNodes(n);
    var a = 1;
    Debug.Assert(s == a, $"Output: {s}\n Expected: {a}");
}

int CountNodes(NaryTree.TreeNode root)
{
    if (root is null) { return 0; }
    (var left, var right) = (root, root);
    (var lcount, var rcount) = (0, 0);
    while (left is not null)
    {
        left = left.left;
        lcount += 1;
    }
    while (right is not null)
    {
        right = right.right;
        rcount += 1;
    }
    if (lcount == rcount) { return (1 << lcount) - 1; }
    return 1 + CountNodes(root.left) + CountNodes(root.right);
}
