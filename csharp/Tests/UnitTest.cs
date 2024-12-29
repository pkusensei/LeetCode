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
        var a = TreeNode.Make([6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5]);
        Assert.AreEqual(18, sol.SumEvenGrandparent(a));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1]);
        Assert.AreEqual(0, sol.SumEvenGrandparent(a));
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