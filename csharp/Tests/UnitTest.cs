using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(32, 12, new[] { 2, 7, 13, 19 })]
    [DataRow(1, 1, new[] { 2, 3, 5 })]
    public void TestMethod1(int exp, int n, int[] p)
    {
        Assert.AreEqual(exp, sol.NthSuperUglyNumber(n, p));
        Assert.AreEqual(exp, sol.WithDp(n, p));
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