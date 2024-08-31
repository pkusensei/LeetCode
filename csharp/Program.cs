using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = Tree.TreeNode.Make([4, 2, 7, 1, 3, 6, 9]);
    var s = InvertTree(n);
    var a = "[4,7,2,9,6,3,1]";
    Debug.Assert(s.ToString() == a, $"Output: {s}\n Expected: {a}");
}

void Test2()
{
    var n = Tree.TreeNode.Make([2, 1, 3]);
    var s = InvertTree(n);
    var a = "[2,3,1]";
    Debug.Assert(s.ToString() == a, $"Output: {s}\n Expected: {a}");
}

Tree.TreeNode InvertTree(Tree.TreeNode root)
{
    if (root is null) { return null; }
    (var left, var right) = (InvertTree(root.right), InvertTree(root.left));
    root.left = left;
    root.right = right;
    return root;
}
