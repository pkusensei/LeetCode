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
        int[][] m = [[1, 0, 1], [0, -2, 3]];
        Assert.AreEqual(2, sol.MaxSumSubmatrix(m, 2));
        Assert.AreEqual(2, sol.WithKadanes(m, 2));
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] m = [[2, 2, -1]];
        Assert.AreEqual(3, sol.MaxSumSubmatrix(m, 3));
        Assert.AreEqual(3, sol.WithKadanes(m, 3));
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