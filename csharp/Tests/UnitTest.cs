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
        var a = TreeNode.Make([1, 4, 3, 2, 4, 2, 5, null, null, null, null, null, null, 4, 6]);
        Assert.AreEqual(20, sol.MaxSumBST(a));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([4, 3, null, 1, 2]);
        Assert.AreEqual(2, sol.MaxSumBST(a));
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([-4, -2, -5]);
        Assert.AreEqual(0, sol.MaxSumBST(a));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}