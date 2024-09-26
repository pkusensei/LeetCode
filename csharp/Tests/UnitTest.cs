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
        var a = TreeNode.Make([5, 2, -3]);
        var b = sol.FindFrequentTreeSum(a);
        var c = "[2,-3,4]";
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([5, 2, -5]);
        var b = sol.FindFrequentTreeSum(a);
        var c = "[2]";
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