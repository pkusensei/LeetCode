using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 2, 5 }, new[] { 3, 4 }, 2, 8)]
    [DataRow(new[] { -4, -2, 0, 3 }, new[] { 2, 4 }, 6, 0)]
    [DataRow(new[] { -2, -1, 0, 1, 2 }, new[] { -3, -1, 2, 4, 5 }, 3, -6)]
    public void TestMethod1(int[] a, int[] b, int k, int exp)
    {
        Assert.AreEqual(exp, sol.KthSmallestProduct(a, b, k));
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