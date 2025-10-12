using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(991600007, 5, 5, new[] { 1, 10, 100, 10000, 1000000 })]
    public void TestMethod1(int exp, int m, int k, int[] p)
    {
        Assert.AreEqual(exp, sol.MagicalSum(m, k, p));
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