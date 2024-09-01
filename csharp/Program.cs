using System.Diagnostics;
using LList;
using Tree;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = TreeNode.Make([6, 2, 8, 0, 4, 7, 9, null, null, 3, 5]);
    var s = LowestCommonAncestor(n, new(2), new(8));
    var a = 6;
    Debug.Assert(s.val == a, $"Output: {s}\nExpect: {a}");
}

void Test2()
{
    var n = TreeNode.Make([6, 2, 8, 0, 4, 7, 9, null, null, 3, 5]);
    var s = LowestCommonAncestor(n, new(2), new(4));
    var a = 2;
    Debug.Assert(s.val == a, $"Output: {s}\nExpect: {a}");
}

TreeNode LowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q)
{
    var curr = root;
    var diffp = p.val - curr.val;
    var diffq = q.val - root.val;
    if (diffp == 0 || diffq == 0 || (diffp < 0) != (diffq < 0)) { return curr; }
    if (diffp < 0) { return LowestCommonAncestor(curr.left, p, q); }
    else { return LowestCommonAncestor(curr.right, p, q); }
}