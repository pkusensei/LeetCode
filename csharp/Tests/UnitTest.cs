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
        var a = TreeNode.Make([3, 9, 20, null, null, 15, 7]);
        var b = sol.SumOfLeftLeaves(a);
        var c = 24;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1]);
        var b = sol.SumOfLeftLeaves(a);
        var c = 0;
        Assert.AreEqual(c, b);
    }
}