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
        var a = TreeNode.Make([3, 4, 5, 1, 2]);
        var b = TreeNode.Make([4, 1, 2]);
        Assert.IsTrue(sol.IsSubtree(a, b));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([3, 4, 5, 1, 2, null, null, null, null, 0]);
        var b = TreeNode.Make([4, 1, 2]);
        Assert.IsFalse(sol.IsSubtree(a, b));
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }

}