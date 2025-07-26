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
        int[][] cp = [[2, 3], [1, 4]];
        Assert.AreEqual(9, sol.MaxSubarrays(4, cp));
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] cp = [[1, 2], [2, 5], [3, 5]];
        Assert.AreEqual(12, sol.MaxSubarrays(5, cp));
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