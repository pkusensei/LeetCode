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
        int[][] m = [[1, 5, 9], [10, 11, 13], [12, 13, 15]];
        Assert.AreEqual(13, sol.KthSmallest(m, 8));
        Assert.AreEqual(13, sol.WithBinarySearch(m, 8));
    }

    [TestMethod]
    public void TestMethod2()
    {
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