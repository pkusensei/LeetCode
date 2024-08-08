using System.Diagnostics;

Test1();
Test2();
Test3();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = TreeNode.Make([3, 9, 20, null, null, 15, 7]);
    Debug.Assert(IsBalanced(n));
}

void Test2()
{
    var n = TreeNode.Make([1, 2, 2, 3, 3, null, null, 4, 4]);
    Debug.Assert(!IsBalanced(n));
}

void Test3()
{
    TreeNode n = null;
    Debug.Assert(IsBalanced(n));
}

bool IsBalanced(TreeNode root)
{
    return Solve(root, 0).Item1;
}

(bool, int) Solve(TreeNode root, int depth)
{
    if (root is null) { return (true, depth); }
    (var l, var ldepth) = Solve(root.left, depth + 1);
    (var r, var rdepth) = Solve(root.right, depth + 1);
    var maxDepth = Math.Max(ldepth, rdepth);
    if (l && r)
    {
        if (Math.Abs(ldepth - rdepth) <= 1) { return (true, maxDepth); }
        else { return (false, maxDepth); }
    }
    else { return (false, maxDepth); }
}