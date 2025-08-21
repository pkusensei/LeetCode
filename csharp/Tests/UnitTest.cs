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
        int[][] m = [[1, 0, 1], [1, 1, 0], [1, 1, 0]];
        Assert.AreEqual(13, sol.NumSubmat(m));
        Assert.AreEqual(13, sol.WithMonoStack(m));
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] m = [[0, 1, 1, 0], [0, 1, 1, 1], [1, 1, 1, 0]];
        Assert.AreEqual(24, sol.NumSubmat(m));
        Assert.AreEqual(24, sol.WithMonoStack(m));
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