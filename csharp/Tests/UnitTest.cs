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
        var a = TreeNode.Make([4, 2, 6, 1, 3]);
        var b = sol.GetMinimumDifference(a);
        var c = 1;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, 0, 48, null, null, 12, 49]);
        var b = sol.GetMinimumDifference(a);
        var c = 1;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([236, 104, 701, null, 227, null, 911]);
        var b = sol.GetMinimumDifference(a);
        var c = 9;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod4()
    {
    }

}