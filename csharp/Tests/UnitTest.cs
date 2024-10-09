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
        var a = TreeNode.Make([2, 2, 5, null, null, 5, 7]);
        var b = sol.FindSecondMinimumValue(a);
        var c = 5;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([2, 2, 2]);
        var b = sol.FindSecondMinimumValue(a);
        var c = -1;
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