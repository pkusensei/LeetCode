using System.Diagnostics;
using System.Text;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    TreeNode n = new(5, new(4), new(6, new(3), new(7)));
    Debug.Assert(!IsValidBST(n));
}

void Test2()
{
    TreeNode n = new(2, new(1), new(3));
    Debug.Assert(IsValidBST(n));
}

bool IsValidBST(TreeNode root)
{
    return Solve(root, long.MinValue, long.MaxValue);
}

bool Solve(TreeNode root, long small, long large)
{
    if (root is null) { return true; }

    if (small < root.val && root.val < large)
    {
        return Solve(root.left, small, root.val) && Solve(root.right, root.val, large);
    }
    return false;
}

string Print<T>(IList<T> values)
{
    var sb = new StringBuilder();
    sb.Append('[');
    sb.AppendJoin(',', values);
    sb.Append([']']);
    return sb.ToString();
}
