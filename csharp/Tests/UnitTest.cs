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
        var a1 = TreeNode.Make([1, 3, 2, 5]);
        var a2 = TreeNode.Make([2, 1, 3, null, 4, null, 7]);
        var b = sol.MergeTrees(a1, a2);
        var c = "[3,4,5,5,4,null,7]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a1 = TreeNode.Make([1]);
        var a2 = TreeNode.Make([1, 2]);
        var b = sol.MergeTrees(a1, a2);
        var c = "[2,2]";
        Assert.AreEqual(c, b.ToString());
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