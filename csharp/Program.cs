using System.Diagnostics;
using LList;
using Tree;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = TreeNode.Make([1, 2, 3, null, 5]);
    var s = BinaryTreePaths(n);
    var a = "[\"1->2->5\",\"1->3\"]";
    Debug.Assert(s.Print() == a, $"Output: {s.Print()}\nExpect: {a}");
}

void Test2()
{
    var n = TreeNode.Make([1]);
    var s = BinaryTreePaths(n);
    var a = "[\"1\"]";
    Debug.Assert(s.Print() == a, $"Output: {s.Print()}\nExpect: {a}");
}

IList<string> BinaryTreePaths(TreeNode root)
{
    return Dfs(root).Select(lst =>
    {
        lst.Reverse();
        var sb = new System.Text.StringBuilder();
        sb.AppendJoin("->", lst);
        return sb.ToString();
    }).ToList();
}

List<List<int>> Dfs(TreeNode node)
{
    if (node.left is null && node.right is null) { return [[node.val]]; }
    var res = new List<List<int>>();
    if (node.left is not null)
    {
        res = res.Concat(Dfs(node.left)).ToList();
    }
    if (node.right is not null)
    {
        res = res.Concat(Dfs(node.right)).ToList();
    }
    return res.Select(lst => { lst.Add(node.val); return lst; }).ToList();
}