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
        var a = TreeNode.Make([1, 2, 3, 4, 5, null, 6, 7, null, null, null, null, 8]);
        Assert.AreEqual(15, sol.DeepestLeavesSum(a));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5]);
        Assert.AreEqual(19, sol.DeepestLeavesSum(a));
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