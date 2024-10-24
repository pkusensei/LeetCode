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
        var a1 = TreeNode.Make([1, 2, 3, 4, 5, 6, null, null, null, 7, 8]);
        var a2 = TreeNode.Make([1, 3, 2, null, 6, 4, 5, null, null, null, null, 8, 7]);
        Assert.IsTrue(sol.FlipEquiv(a1, a2));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a1 = TreeNode.Make([1]);
        var a2 = TreeNode.Make([]);
        Assert.IsFalse(sol.FlipEquiv(a1, a2));
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