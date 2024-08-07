using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    TreeNode n = TreeNode.Make([1, 2, 2, 3, 4, 4, 3]);
    Debug.Assert(IsSymmetric(n));
}

void Test2()
{
    TreeNode n = TreeNode.Make([1, 2, 2, null, 3, null, 3]);
    Debug.Assert(!IsSymmetric(n));
}

bool IsSymmetric(TreeNode root)
{
    if (root is null) { return true; }
    return Solve(root.left, root.right);
}

bool Solve(TreeNode left, TreeNode right)
{
    if (left is null && right is null) { return true; }
    if (left is null && right is not null || left is not null && right is null) { return false; }
    return left.val == right.val && Solve(left.left, right.right) && Solve(left.right, right.left);
}