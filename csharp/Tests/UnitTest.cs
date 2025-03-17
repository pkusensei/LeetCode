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
        var a = TreeNode.Make([1, 5, 3, null, 4, 10, 6, 9, 2]);
        Assert.AreEqual(4, sol.AmountOfTime(a, 3));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1]);
        Assert.AreEqual(0, sol.AmountOfTime(a, 1));
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