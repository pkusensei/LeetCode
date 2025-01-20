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
        var a = TreeNode.Make([1, 2, 3, null, 4]);
        Assert.AreEqual(1, sol.CountPairs(a, 3));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, 2, 3, 4, 5, 6, 7]);
        Assert.AreEqual(2, sol.CountPairs(a, 3));
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([7, 1, 4, 6, null, 5, 3, null, null, null, null, null, 2]);
        Assert.AreEqual(1, sol.CountPairs(a, 3));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}