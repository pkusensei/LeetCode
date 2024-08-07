using System.Diagnostics;
using System.Text;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    TreeNode n = new(1, new(3, null, new(2)));
    RecoverTree(n);
    var a = "[3,1,null,null,2]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}

void Test2()
{
    TreeNode n = new(5, new(3, new(-2147483648), new(2)), new(9));
    RecoverTree(n);
    var a = "[5,2,9,-2147483648,3]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");

}

void RecoverTree(TreeNode root)
{
    (TreeNode fst, TreeNode snd) = (null, null);
    TreeNode prev = new(int.MinValue);
    Solve(root, ref fst, ref snd, ref prev);
    (snd.val, fst.val) = (fst.val, snd.val);
}

void Solve(TreeNode node, ref TreeNode fst, ref TreeNode snd, ref TreeNode prev)
{
    if (node is null) { return; }

    Solve(node.left, ref fst, ref snd, ref prev);
    if (fst is null && prev.val > node.val)
    {
        fst = prev;
    }
    if (fst is not null && prev.val > node.val)
    {
        snd = node;
    }
    prev = node;
    Solve(node.right, ref fst, ref snd, ref prev);
}

string Print<T>(IList<T> values)
{
    var sb = new StringBuilder();
    sb.Append('[');
    sb.AppendJoin(',', values);
    sb.Append([']']);
    return sb.ToString();
}
