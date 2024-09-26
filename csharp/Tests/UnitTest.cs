using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        var a = TreeNode.Make([1, null, 2, 2]);
        var b = sol.FindMode(a);
        var c = "[2]";
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([0]);
        var b = sol.FindMode(a);
        var c = "[0]";
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