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
        var a = TreeNode.Make([1, null, 1, 1, 1, null, null, 1, 1, null, 1, null, null, null, 1]);
        Assert.AreEqual(3, sol.LongestZigZag(a));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, 1, 1, null, 1, null, null, 1, 1, null, 1]);
        Assert.AreEqual(4, sol.LongestZigZag(a));
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1]);
        Assert.AreEqual(0, sol.LongestZigZag(a));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}