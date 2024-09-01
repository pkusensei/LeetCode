using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = Tree.TreeNode.Make([3, 1, 4, null, 2]);
    var s = KthSmallest(n, 1);
    var a = 1;
    Debug.Assert(s == a, $"Output: {s}\nExpect: {a}");
}

void Test2()
{
    var n = Tree.TreeNode.Make([5, 3, 6, 2, 4, null, null, 1]);
    var s = KthSmallest(n, 3);
    var a = 3;
    Debug.Assert(s == a, $"Output: {s}\nExpect: {a}");
}

int KthSmallest(Tree.TreeNode root, int k)
{
    var stack = new Stack<Tree.TreeNode>();
    var curr = root;
    var i = 0;
    while (stack.Count > 0 || curr is not null)
    {
        if (curr is not null)
        {
            stack.Push(curr);
            curr = curr.left;
        }
        else
        {
            var temp = stack.Pop();
            i += 1;
            if (i == k) { return temp.val; }
            curr = temp.right;
        }
    }
    return 0;
}