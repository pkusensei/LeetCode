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
        var a = TreeNode.Make([1, null, 0, 0, 1]);
        var b = sol.PruneTree(a);
        var c = "[1,null,0,null,1]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, 0, 1, 0, 0, 0, 1]);
        var b = sol.PruneTree(a);
        var c = "[1,null,1,null,1]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1, 1, 0, 1, 1, 0, 1, 0]);
        var b = sol.PruneTree(a);
        var c = "[1,1,0,1,1,null,1]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod4()
    {
    }

}