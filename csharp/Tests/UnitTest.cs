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
        int[][] a = [[3, 2], [4, 3], [4, 4], [2, 5]];
        Assert.AreEqual(5, sol.MostPoints(a));
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] a = [[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]];
        Assert.AreEqual(7, sol.MostPoints(a));
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