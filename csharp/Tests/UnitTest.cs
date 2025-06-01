using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(5, 2, 3)]
    [DataRow(3, 3, 10)]
    public void TestMethod1(int n, int limit, int exp)
    {
        Assert.AreEqual(exp, sol.DistributeCandies(n, limit));
        Assert.AreEqual(exp, sol.PIE(n, limit));
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