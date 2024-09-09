using System.Diagnostics;
using System.Text;
using LList;
using Tree;

Test1();
Test2();
Test3();
Test4();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var a = TreeNode.Make([3, 2, 3, null, 3, null, 1]);
    var b = Rob(a);
    var c = 7;
    Debug.Assert(b == c, $"Output: {b}\nExpect: {c}");
}

void Test2()
{
    var a = TreeNode.Make([3, 4, 5, 1, 3, null, 1]);
    var b = Rob(a);
    var c = 9;
    Debug.Assert(b == c, $"Output: {b}\nExpect: {c}");
}

void Test3()
{
    var a = TreeNode.Make([4, 1, null, 2, null, 3]);
    var b = Rob(a);
    var c = 7;
    Debug.Assert(b == c, $"Output: {b}\nExpect: {c}");
}

void Test4()
{
    var a = TreeNode.Make([4, 2, null, 1, 3]);
    var b = Rob(a);
    var c = 8;
    Debug.Assert(b == c, $"Output: {b}\nExpect: {c}");
}

int Rob(TreeNode root)
{
    var res = Solve(root);
    return Math.Max(res.inc, res.exc);
}

(int inc, int exc) Solve(TreeNode root)
{
    if (root is null) { return (0, 0); }
    var left = Solve(root.left);
    var right = Solve(root.right);
    var inc = left.exc + right.exc + root.val;
    var exc = Math.Max(left.inc, left.exc) + Math.Max(right.inc, right.exc);
    return (inc, exc);
}