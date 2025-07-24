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
        int[][] m = [[1, 4, 7, 11, 15], [2, 5, 8, 12, 19], [3, 6, 9, 16, 22], [10, 13, 14, 17, 24], [18, 21, 23, 26, 30]];
        Assert.IsTrue(sol.SearchMatrix(m, 5));
        Assert.IsTrue(sol.WithLinearTime(m, 5));
    }

    [TestMethod]
    public void TestMethod2()
    {
        Assert.IsTrue(sol.SearchMatrix([[-1, 3]], 3));
        Assert.IsTrue(sol.WithLinearTime([[-1, 3]], 3));
    }

    [TestMethod]
    public void TestMethod3()
    {
        int[][] m = [[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15], [16, 17, 18, 19, 20], [21, 22, 23, 24, 25]];
        Assert.IsTrue(sol.SearchMatrix(m, 19));
        Assert.IsTrue(sol.WithLinearTime(m, 19));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}