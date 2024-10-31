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
        var b = sol.SubtreeWithAllDeepest(a);
        var c = "[2,7,4]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1]);
        var b = sol.SubtreeWithAllDeepest(a);
        var c = "[1]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([0, 1, 3, null, 2]);
        var b = sol.SubtreeWithAllDeepest(a);
        var c = "[2]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}