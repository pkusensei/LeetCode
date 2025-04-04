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
        Assert.AreEqual("[2,7,4]", sol.LcaDeepestLeaves(a).ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([0, 1, 3, null, 2]);
        Assert.AreEqual("[2]", sol.LcaDeepestLeaves(a).ToString());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1]);
        Assert.AreEqual("[1]", sol.LcaDeepestLeaves(a).ToString());
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}