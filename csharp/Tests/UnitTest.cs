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
        var a = TreeNode.Make([1, 2, 3]);
        var b = sol.FindTilt(a);
        var c = 1;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([4, 2, 9, 3, 5, null, 7]);
        var b = sol.FindTilt(a);
        var c = 15;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([21, 7, 14, 1, 1, 2, 2, 3, 3]);
        var b = sol.FindTilt(a);
        var c = 9;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod4()
    {
    }

}