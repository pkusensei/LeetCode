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
        var root = TreeNode.Make([3, 2, 3, null, 3, null, 1]);
        Assert.AreEqual(7, sol.Rob(root));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var root = TreeNode.Make([3, 4, 5, 1, 3, null, 1]);
        Assert.AreEqual(9, sol.Rob(root));
    }

    [TestMethod]
    public void TestMethod3()
    {
        var root = TreeNode.Make([4, 1, null, 2, null, 3]);
        Assert.AreEqual(7, sol.Rob(root));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}