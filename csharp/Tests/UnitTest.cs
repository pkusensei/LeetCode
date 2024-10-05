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
        var a = TreeNode.Make([3, 9, 20, null, null, 15, 7]);
        var b = sol.AverageOfLevels(a);
        var c = "[3.00000,14.50000,11.00000]";
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([3, 9, 20, 15, 7]);
        var b = sol.AverageOfLevels(a);
        var c = "[3.00000,14.50000,11.00000]";
        Assert.AreEqual(c, b.Print());
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