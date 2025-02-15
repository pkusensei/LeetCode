using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        List<List<int?>> arrs = [[2, 1], [3, 2, 5], [5, 4]];
        var trees = arrs.Select(nums => TreeNode.Make(nums)).ToList();
        Assert.AreEqual("[3,2,5,1,null,4]", sol.CanMerge(trees).ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        List<List<int?>> arrs = [[5, 3, 8], [3, 2, 6]];
        var trees = arrs.Select(nums => TreeNode.Make(nums)).ToList();
        Assert.IsTrue(sol.CanMerge(trees) is null);
    }

    [TestMethod]
    public void TestMethod3()
    {
        List<List<int?>> arrs = [[5, 4], [3]];
        var trees = arrs.Select(nums => TreeNode.Make(nums)).ToList();
        Assert.IsTrue(sol.CanMerge(trees) is null);
    }

    [TestMethod]
    public void TestMethod4()
    {
        List<List<int?>> arrs = [[1, null, 3], [3, 1], [4, 2]];
        var trees = arrs.Select(nums => TreeNode.Make(nums)).ToList();
        Assert.IsTrue(sol.CanMerge(trees) is null);
    }
}