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
        var a = TreeNode.Make([1, 2]);
        var b = sol.FlipMatchVoyage(a, [2, 1]);
        Assert.AreEqual("[-1]", b.Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, 2, 3]);
        var b = sol.FlipMatchVoyage(a, [1, 3, 2]);
        Assert.AreEqual("[1]", b.Print());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1, 2, 3]);
        var b = sol.FlipMatchVoyage(a, [1, 2, 3]);
        Assert.AreEqual(0, b.Count);
    }

    [TestMethod]
    public void TestMethod4()
    {
        var a = TreeNode.Make([1, null, 2]);
        var b = sol.FlipMatchVoyage(a, [1, 2]);
        Assert.AreEqual(0, b.Count);
    }

    [TestMethod]
    public void TestMethod5()
    {
        var a = TreeNode.Make([1, 2, null, 3]);
        var b = sol.FlipMatchVoyage(a, [1, 3, 2]);
        Assert.AreEqual("[-1]", b.Print());
    }
}