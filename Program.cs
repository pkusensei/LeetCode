using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    TreeNode n = TreeNode.Make([3, 9, 20, null, null, 15, 7]);
    var s = MaxDepth(n);
    var a = 3;
    Debug.Assert(s == a, $"Expect: {s}\nOutput: {a}");
}

void Test2()
{
    TreeNode n = TreeNode.Make([1, null, 2]);
    var s = MaxDepth(n);
    var a = 2;
    Debug.Assert(s == a, $"Expect: {s}\nOutput: {a}");
}

int MaxDepth(TreeNode root)
{
    return Dfs(root, 1);
}

int Dfs(TreeNode root, int depth)
{
    if (root is null) { return depth - 1; }
    return Math.Max(Dfs(root.left, depth + 1), Dfs(root.right, depth + 1));
}