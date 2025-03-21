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
        var a = TreeNode.Make([1, 3, 4, 2, null, 6, 5, null, null, null, null, null, 7]);
        Assert.AreEqual("[2]", sol.TreeQueries(a, [4]).Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([5, 8, 9, 2, 1, 3, 7, 4, 6]);
        Assert.AreEqual("[3,2,3,2]", sol.TreeQueries(a, [3, 2, 4, 8]).Print());
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