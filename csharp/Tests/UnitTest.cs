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
        int[][] a = [[1, 2, 3], [2, 5, 7], [3, 5, 1]];
        Assert.AreEqual("[5,8,1]", sol.MaxPoints(a, [5, 6, 2]).Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] a = [[5, 2, 1], [1, 1, 2]];
        Assert.AreEqual("[0]", sol.MaxPoints(a, [3]).Print());
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