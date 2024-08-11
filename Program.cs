using System.Diagnostics;

Test1();
Test2();

Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = TreeNode.Make([1, 2, 3]);
    var s = SumNumbers(n);
    var a = 25;
    Debug.Assert(s == a, $"Output: {s}\nExpect: {a}");
}

void Test2()
{
    var n = TreeNode.Make([4, 9, 0, 5, 1]);
    var s = SumNumbers(n);
    var a = 1026;
    Debug.Assert(s == a, $"Output: {s}\nExpect: {a}");
}


int SumNumbers(TreeNode root)
{
    return Solve(root, 0);
}

int Solve(TreeNode root, int curr)
{
    if (root.left is null && root.right is null)
    {
        return curr * 10 + root.val;
    }

    var res = 0;
    if (root.left is not null)
    {
        res += Solve(root.left, curr * 10 + root.val);
    }
    if (root.right is not null)
    {
        res += Solve(root.right, curr * 10 + root.val);
    }
    return res;
}