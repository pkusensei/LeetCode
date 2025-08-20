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
        int[][] m = [[0, 1, 1, 1], [1, 1, 1, 1], [0, 1, 1, 1]];
        Assert.AreEqual(15, sol.CountSquares(m));
        Assert.AreEqual(15, sol.WithBetterSpace(m));
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] m = [[1, 0, 1], [1, 1, 0], [1, 1, 0]];
        Assert.AreEqual(7, sol.CountSquares(m));
        Assert.AreEqual(7, sol.WithBetterSpace(m));
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