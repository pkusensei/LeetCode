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
        var a = TreeNode.Make([1, 7, 0, 7, -8, null, null]);
        Assert.AreEqual(2, sol.MaxLevelSum(a));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([989, null, 10250, 98693, -89388, null, null, null, -32127]);
        Assert.AreEqual(2, sol.MaxLevelSum(a));
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