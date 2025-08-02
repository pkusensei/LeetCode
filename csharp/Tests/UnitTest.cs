using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(1, new[] { 4, 2, 2, 2 }, new[] { 1, 4, 1, 2 })]
    [DataRow(-1, new[] { 2, 3, 4, 1 }, new[] { 3, 2, 5, 1 })]
    public void TestMethod1(int exp, int[] a, int[] b)
    {
        Assert.AreEqual(exp, sol.MinCost(a, b));
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