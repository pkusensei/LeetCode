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
        var a = TreeNode.Make([10, 5, -3, 3, 2, null, 11, 3, -2, null, 1]);
        var b = sol.PathSum(a, 8);
        var c = 3;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1]);
        var b = sol.PathSum(a, 22);
        var c = 3;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1, -2, -3, 1, 3, -2, null, -1]);
        var b = sol.PathSum(a, -1);
        var c = 4;
        Assert.AreEqual(c, b);
    }

}