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
        var a = TreeNode.Make([1, null, 2, null, 3, null, 4, null, null]);
        Assert.AreEqual("[3,2,4,1]", sol.BalanceBST(a).ToString());
        Assert.AreEqual("[3,2,4,1]", sol.WithDSW(a).ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([2, 1, 3]);
        Assert.AreEqual("[2,1,3]", sol.BalanceBST(a).ToString());
        Assert.AreEqual("[2,1,3]", sol.WithDSW(a).ToString());
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