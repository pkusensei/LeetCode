using System.Diagnostics;
using LList;
using Tree;

Test1();
Test2();
Test3();
Test4();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var a = ListNode.Make([4, 2, 8]);
    var b = TreeNode.Make([1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3]);
    Debug.Assert(IsSubPath(a, b));
}

void Test2()
{
    var a = ListNode.Make([1, 4, 2, 6]);
    var b = TreeNode.Make([1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3]);
    Debug.Assert(IsSubPath(a, b));
}

void Test3()
{
    var a = ListNode.Make([1, 4, 2, 6, 8]);
    var b = TreeNode.Make([1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3]);
    Debug.Assert(!IsSubPath(a, b));
}

void Test4()
{
    var a = ListNode.Make([4, 1]);
    var b = TreeNode.Make([1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3]);
    Debug.Assert(!IsSubPath(a, b));
}


bool IsSubPath(ListNode head, TreeNode root)
{
    if (root is null) { return false; }
    return Dfs(head, root) || IsSubPath(head, root.left) || IsSubPath(head, root.right);
}

bool Dfs(ListNode head, TreeNode root)
{
    if (head is null) { return true; }
    if (root is null) { return false; }
    if (head.val != root.val) { return false; }
    return Dfs(head.next, root.left) || Dfs(head.next, root.right);
}
