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
        var a = TreeNode.Make([1, 3, 4, 2, null, 6, 5, null, null, null, null, null, 7]);
        var b = sol.TwoLargestCousins(a, [4]);
        var c = "[2]";
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([5, 8, 9, 2, 1, 3, 7, 4, 6]);
        var b = sol.TwoLargestCousins(a, [3, 2, 4, 8]);
        var c = "[3,2,3,2]";
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1, null, 5, 3, null, 2, 4]);
        var b = sol.TwoLargestCousins(a, [3, 5, 4, 2, 4]);
        var c = "[1,0,3,3,3]";
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod4()
    {
    }

}