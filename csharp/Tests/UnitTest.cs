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
        var a = TreeNode.Make([1, 2, 3, 4, null, 2, 4, null, null, 4]);
        var b = sol.FindDuplicateSubtrees(a);
        var c = "[[2,4],[4]]";
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([2, 1, 1]);
        var b = sol.FindDuplicateSubtrees(a);
        var c = "[[1]]";
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([2, 2, 2, 3, null, 3, null]);
        var b = sol.FindDuplicateSubtrees(a);
        var c = "[[2,3],[3]]";
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod4()
    {
    }

}