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
        var t = TreeNode.Make([0, 0, null, 0, 0]);
        var b = sol.WithDp(t);
        Assert.AreEqual(1, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var t = TreeNode.Make([0, 0, null, 0, null, 0, null, null, 0]);
        var b = sol.WithDp(t);
        Assert.AreEqual(2, b);
    }

    [TestMethod]
    public void TestMethod3()
    {
        var t = TreeNode.Make([0]);
        var b = sol.WithDp(t);
        Assert.AreEqual(1, b);
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}