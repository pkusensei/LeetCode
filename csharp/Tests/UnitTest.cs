using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        var a = TreeNode.Make([2, 1, 3]);
        var b = sol.FindBottomLeftValue(a);
        var c = 1;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, 2, 3, 4, null, 5, 6, null, null, 7]);
        var b = sol.FindBottomLeftValue(a);
        var c = 7;
        Assert.AreEqual(c, b);
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