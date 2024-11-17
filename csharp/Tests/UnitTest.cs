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
        var a = TreeNode.Make([1, 2, 3, 4]);
        Assert.IsFalse(sol.IsCousins(a, 4, 3));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, 2, 3, null, 4, null, 5]);
        Assert.IsTrue(sol.IsCousins(a, 5, 4));
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1, 2, 3, null, 4]);
        Assert.IsFalse(sol.IsCousins(a, 2, 3));
    }
}