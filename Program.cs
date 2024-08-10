using System.Diagnostics;

Test1();
Test2();
Test3();
Test4();
Test5();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = TreeNode.Make([1, 2, 3]);
    var s = MaxPathSum(n);
    var a = 6;
    Debug.Assert(s == a, $"Output: {s}\nExpect: {a}");
}

void Test2()
{
    var n = TreeNode.Make([-10, 9, 20, null, null, 15, 7]);
    var s = MaxPathSum(n);
    var a = 42;
    Debug.Assert(s == a, $"Output: {s}\nExpect: {a}");
}

void Test3()
{
    var n = TreeNode.Make([1, 2]);
    var s = MaxPathSum(n);
    var a = 3;
    Debug.Assert(s == a, $"Output: {s}\nExpect: {a}");
}

void Test4()
{
    var n = TreeNode.Make([2, -1]);
    var s = MaxPathSum(n);
    var a = 2;
    Debug.Assert(s == a, $"Output: {s}\nExpect: {a}");
}

void Test5()
{
    var n = TreeNode.Make([1, -2, -3, 1, 3, -2, null, -1]);
    var s = MaxPathSum(n);
    var a = 3;
    Debug.Assert(s == a, $"Output: {s}\nExpect: {a}");
}


int MaxPathSum(TreeNode root)
{
    return Solve(root).treeSum;
}

(int branchSum, int treeSum) Solve(TreeNode root)
{
    if (root is null) { return (0, int.MinValue); }

    (var lb, var lt) = Solve(root.left);
    (var rb, var rt) = Solve(root.right);
    (lb, rb) = (Math.Max(0, lb), Math.Max(0, rb));

    var treeSum = Math.Max(Math.Max(lt, rt), root.val + lb + rb);
    return (Math.Max(lb, rb) + root.val, treeSum);
}