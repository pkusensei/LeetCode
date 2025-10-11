using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(10, new[] { 7, 1, 6, 3 })]
    public void TestMethod1(long exp, int[] p)
    {
        Assert.AreEqual(exp, sol.MaximumTotalDamage(p));
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