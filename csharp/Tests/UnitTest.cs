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
        int[][] m = [[0, 4], [4, 4]];
        Assert.AreEqual(7, sol.MinTimeToReach(m));
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] m = [[0, 0, 0, 0], [0, 0, 0, 0]];
        Assert.AreEqual(6, sol.MinTimeToReach(m));
    }

    [TestMethod]
    public void TestMethod3()
    {
        int[][] m = [[0, 1], [1, 2]];
        Assert.AreEqual(4, sol.MinTimeToReach(m));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}