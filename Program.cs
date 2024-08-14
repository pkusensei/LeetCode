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
    var s = PostorderTraversal(n);
    var a = "[3,2,1]";
    Debug.Assert(Print(s) == a, $"Output: {Print(s)}\nExpect: {a}");
}

void Test2()
{
    var n = TreeNode.Make([]);
    var s = PostorderTraversal(n);
    var a = "[]";
    Debug.Assert(Print(s) == a, $"Output: {Print(s)}\nExpect: {a}");
}

void Test3()
{
    var n = TreeNode.Make([1]);
    var s = PostorderTraversal(n);
    var a = "[1]";
    Debug.Assert(Print(s) == a, $"Output: {Print(s)}\nExpect: {a}");
}

void Test4()
{
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

IList<int> PostorderTraversal(TreeNode root)
{
    var res = new List<int>();
    if (root is null) { return res; }
    var stack = new Stack<TreeNode>();
    var curr = root;
    TreeNode last = null;
    while (curr is not null || stack.Count > 0)
    {
        if (curr is not null)
        {
            stack.Push(curr);
            curr = curr.left;
        }
        else
        {
            var peek = stack.Peek();
            if (peek.right is not null && last != peek.right)
            {
                curr = peek.right;
            }
            else
            {
                res.Add(peek.val);
                last = stack.Pop();
            }
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