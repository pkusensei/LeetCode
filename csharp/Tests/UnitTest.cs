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
        var a = TreeNode.Make([3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]);
        var b = sol.DistanceK(a, a.left, 2);
        var c = "[7,4,1]";
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1]);
        var b = sol.DistanceK(a, a, 3);
        Assert.AreEqual(0, b.Count);
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([0, 1, null, 3, 2]);
        var b = sol.DistanceK(a, a.left.right, 1);
        var c = "[1]";
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod4()
    {
    }

}