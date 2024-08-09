using System.Diagnostics;

Test1();
Test2();
Test3();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = TreeNode.Make([5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1]);
    var r = PathSum(n, 22);
    IList<IList<int>> a = [[5, 4, 11, 2], [5, 8, 4, 5]];
    Debug.Assert(r == a, $"Output: {r}\nExpect: {a}");
}

void Test2()
{
    var n = TreeNode.Make([1, 2, 3]);
    var r = PathSum(n, 5);
    List<IList<int>> a = [];
    Debug.Assert(r == a, $"Output: {r}\nExpect: {a}");
}

void Test3()
{
    var n = TreeNode.Make([]);
    var r = PathSum(n, 1);
    List<IList<int>> a = [];
    Debug.Assert(r == a, $"Output: {r}\nExpect: {a}");
}

IList<IList<int>> PathSum(TreeNode root, int targetSum)
{
    if (root is null) { return []; }

    var target = targetSum - root.val;
    if (root.left is null && root.right is null && target == 0)
    {
        return [[root.val]];
    }
    var res = new List<IList<int>>();
    foreach (var item in PathSum(root.left, target).Concat(PathSum(root.right, target)))
    {
        if (item is not null)
        {
            item.Insert(0, root.val);
            res.Add(item);
        }
    }
    return res;
}