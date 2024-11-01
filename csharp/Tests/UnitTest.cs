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
        var a1 = TreeNode.Make([3, 5, 1, 6, 2, 9, 8, null, null, 7, 4]);
        var a2 = TreeNode.Make([3, 5, 1, 6, 7, 4, 2, null, null, null, null, null, null, 9, 8]);
        Assert.IsTrue(sol.LeafSimilar(a1, a2));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a1 = TreeNode.Make([1, 2, 3]);
        var a2 = TreeNode.Make([1, 3, 2]);
        Assert.IsFalse(sol.LeafSimilar(a1, a2));
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