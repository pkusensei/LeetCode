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
        var a = TreeNode.Make([5, 3, 6, 2, 4, null, 7]);
        Assert.IsTrue(sol.FindTarget(a, 9));
        // var b = sol.FindDuplicateSubtrees(a);
        // var c = "[[2,4],[4]]";
        // Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([5, 3, 6, 2, 4, null, 7]);
        Assert.IsFalse(sol.FindTarget(a, 28));
        // var b = sol.FindDuplicateSubtrees(a);
        // var c = "[[1]]";
        // Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([2, 1, 3]);
        Assert.IsTrue(sol.FindTarget(a, 4));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }

}