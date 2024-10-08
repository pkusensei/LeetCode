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
        var a = TreeNode.Make([1, 3, 2, 5, null, null, 9, 6, null, 7]);
        var b = sol.WidthOfBinaryTree(a);
        var c = 7;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, 3, 2, 5, 3, null, 9]);
        var b = sol.WidthOfBinaryTree(a);
        var c = 4;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1, 3, 2, 5]);
        var b = sol.WidthOfBinaryTree(a);
        var c = 2;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod4()
    {
    }

}