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
        int[][] a = [[1, 2], [2, 3], [5], [0], [5], [], []];
        Assert.AreEqual("[2,4,5,6]", sol.EventualSafeNodes(a).Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] a = [[1, 2, 3, 4], [1, 2], [3, 4], [0, 4], []];
        Assert.AreEqual("[4]", sol.EventualSafeNodes(a).Print());
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