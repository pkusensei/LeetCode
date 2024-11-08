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
        var a = TreeNode.Make([10, 5, 15, 3, 7, null, 18]);
        var b = sol.RangeSumBST(a, 7, 15);
        Assert.AreEqual(32, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([10, 5, 15, 3, 7, 13, 18, 1, null, 6]);
        var b = sol.RangeSumBST(a, 6, 10);
        Assert.AreEqual(23, b);
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