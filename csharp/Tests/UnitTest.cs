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
        var a = TreeNode.Make([5, 1, 2, 3, null, 6, 4]);
        Assert.AreEqual("UURL", sol.GetDirections(a, 3, 6));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([2, 1]);
        Assert.AreEqual("L", sol.GetDirections(a, 2, 1));
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