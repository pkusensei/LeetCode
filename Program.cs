using System.Diagnostics;
using System.Text;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    TreeNode root = new(1, null, new(2, new(3)));
    var n = Print(InorderTraversal(root));
    var a = "[1,3,2]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}

void Test2()
{
}


IList<int> InorderTraversal(TreeNode root)
{
    if (root is null)
    {
        return [];
    }

    return root.InorderTrav();
}

string Print<T>(IList<T> values)
{
    var sb = new StringBuilder();
    sb.Append('[');
    sb.AppendJoin(',', values);
    sb.Append([']']);
    return sb.ToString();
}
