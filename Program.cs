using System.Diagnostics;
using System.Text;

Test1();
Test2();
Test3();
Test4();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = TreeNode.Make([1, null, 2, 3]);
    var s = PreorderTraversal(n);
    var a = "[1,2,3]";
    Debug.Assert(Print(s) == a, $"Output: {Print(s)}\nExpect: {a}");
}

void Test2()
{
    var n = TreeNode.Make([]);
    var s = PreorderTraversal(n);
    var a = "[]";
    Debug.Assert(Print(s) == a, $"Output: {Print(s)}\nExpect: {a}");
}

void Test3()
{
    var n = TreeNode.Make([1]);
    var s = PreorderTraversal(n);
    var a = "[1]";
    Debug.Assert(Print(s) == a, $"Output: {Print(s)}\nExpect: {a}");
}

void Test4()
{
    var n = TreeNode.Make([1, 4, 3, 2]);
    var s = PreorderTraversal(n);
    var a = "[1,4,2,3]";
    Debug.Assert(Print(s) == a, $"Output: {Print(s)}\nExpect: {a}");
}

IList<int> PreorderTraversal(TreeNode root)
{
    var res = new List<int>();
    if (root is null) { return res; }

    var stack = new Stack<TreeNode>();
    stack.Push(root);
    while (stack.TryPop(out var node))
    {
        if (node is not null)
        {
            res.Add(node.val);
            stack.Push(node.right);
            stack.Push(node.left);
        }
    }
    return res;
}

string Print(IEnumerable<int> values)
{
    var sb = new StringBuilder();
    sb.Append('[');
    foreach (var item in values)
    {
        sb.AppendFormat("{0},", item);
    }
    if (sb.Length > 1)
    {
        sb.Replace(',', ']', sb.Length - 1, 1);
    }
    else
    {
        sb.Append(']');
    }
    return sb.ToString();
}