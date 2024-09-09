using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        var a = TreeNode.Make([3, 2, 3, null, 3, null, 1]);
        var b = sol.Rob(a);
        var c = 7;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([3, 4, 5, 1, 3, null, 1]);
        var b = sol.Rob(a);
        var c = 9;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([4, 2, null, 1, 3]);
        var b = sol.Rob(a);
        var c = 8;
        Assert.AreEqual(c, b);
    }
}