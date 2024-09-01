using System.Diagnostics;
using LList;
using Tree;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = TreeNode.Make([3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]);
    var s = LowestCommonAncestor(n, new(5), new(1));
    var a = 3;
    Debug.Assert(s.val == a, $"Output: {s}\nExpect: {a}");
}

void Test2()
{
    var n = TreeNode.Make([3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]);
    var s = LowestCommonAncestor(n, new(5), new(4));
    var a = 5;
    Debug.Assert(s.val == a, $"Output: {s}\nExpect: {a}");
}

TreeNode LowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q)
{
        if (root is null) { return null; }
        if (root.val == p.val || root.val == q.val) { return root; }

        var left = LowestCommonAncestor(root.left, p, q);
        var right = LowestCommonAncestor(root.right, p, q);
        if (left is not null && right is not null) { return root; }
        else if (left is not null) { return left; }
        else { return right; }
}