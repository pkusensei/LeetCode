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
        var a = TreeNode.Make([4, 8, 5, 0, 1, null, 6]);
        Assert.AreEqual(5, sol.AverageOfSubtree(a));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1]);
        Assert.AreEqual(1, sol.AverageOfSubtree(a));
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