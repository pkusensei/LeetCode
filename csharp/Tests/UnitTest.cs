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
        var a = TreeNode.Make([10, 5, -3, 3, 2, null, 11, 3, -2, null, 1]);
        Assert.AreEqual(3, sol.PathSum(a, 8));
        Assert.AreEqual(3, sol.WithPrefixSum(a, 8));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1]);
        Assert.AreEqual(3, sol.PathSum(a, 22));
        Assert.AreEqual(3, sol.WithPrefixSum(a, 22));
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