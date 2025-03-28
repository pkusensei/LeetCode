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
        int[][] a = [[0, 1, 3, 2], [5, 1, 2, 5], [4, 3, 8, 6]];
        Assert.AreEqual(7, sol.MinimumTime(a));
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] a = [[0, 2, 4], [3, 2, 1], [1, 0, 4]];
        Assert.AreEqual(-1, sol.MinimumTime(a));
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