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
        var a = TreeNode.Make([1, 4, 3, 7, 6, 8, 5, null, null, null, null, 9, null, 10]);
        Assert.AreEqual(3, sol.MinimumOperations(a));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.MakeInt([1, 3, 2, 7, 6, 5, 4]);
        Assert.AreEqual(3, sol.MinimumOperations(a));
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1, 2, 3, 4, 5, 6]);
        Assert.AreEqual(0, sol.MinimumOperations(a));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}