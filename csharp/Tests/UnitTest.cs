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
        var a = TreeNode.Make([3, 0, 0]);
        var b = sol.DistributeCoins(a);
        Assert.AreEqual(2, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([0, 3, 0]);
        var b = sol.DistributeCoins(a);
        Assert.AreEqual(3, b);
    }

    [TestMethod]
    public void TestMethod3()
    {
    }
}