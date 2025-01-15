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
        var a = TreeNode.Make([3, 1, 4, 3, null, 1, 5]);
        Assert.AreEqual(4, sol.GoodNodes(a));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([3, 3, null, 4, 2]);
        Assert.AreEqual(3, sol.GoodNodes(a));
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1]);
        Assert.AreEqual(1, sol.GoodNodes(a));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}