using System.Diagnostics;

Test1();
Test2();
Test3();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = TreeNode.Make([5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1]);
    Debug.Assert(HasPathSum(n, 22));
}

void Test2()
{
    var n = TreeNode.Make([1, 2, 3]);
    Debug.Assert(!HasPathSum(n, 5));
    // var s = MinDepth(n);
    // var a = 5;
    // Debug.Assert(s == a, $"Output: {s}\nExpected:{a}");
}

void Test3()
{
    var n = TreeNode.Make([]);
    Debug.Assert(!HasPathSum(n, 0));
}

bool HasPathSum(TreeNode root, int targetSum)
{
    if (root is null) { return false; }

    var target = targetSum - root.val;
    if (root.left is null && root.right is null)
    {
        return target == 0;
    }
    return HasPathSum(root.left, target) || HasPathSum(root.right, target);
}