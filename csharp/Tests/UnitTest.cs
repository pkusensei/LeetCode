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
        int[][] a = [[0, 1], [0, 2], [0, 3]];
        int[][] b = [[0, 1]];
        Assert.AreEqual(3, sol.MinimumDiameterAfterMerge(a, b));
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] a = [[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]];
        int[][] b = [[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]];
        Assert.AreEqual(5, sol.MinimumDiameterAfterMerge(a, b));
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