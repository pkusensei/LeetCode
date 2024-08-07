using System.Diagnostics;
using System.Text;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    TreeNode p = TreeNode.Make([1, 2, 3]);
    TreeNode q = TreeNode.Make([1, 2, 3]);
    Debug.Assert(IsSameTree(p, q));
}

void Test2()
{
    TreeNode p = TreeNode.Make([1, 2, 1]);
    TreeNode q = TreeNode.Make([1, 1, 2]);
    Debug.Assert(!IsSameTree(p, q));
}

bool IsSameTree(TreeNode p, TreeNode q)
{
    if (p is null && q is null) { return true; }
    else if (p is null && q is not null || p is not null && q is null) { return false; }
    else
    {
        return p.val == q.val && IsSameTree(p.left, q.left) && IsSameTree(p.right, q.right);
    }
}