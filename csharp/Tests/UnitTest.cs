using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(13, 2, 10)]
    [DataRow(1, 1, 1)]
    [DataRow(100, 10, 17)]
    public void TestMethod1(int n, int k, int exp)
    {
        Assert.AreEqual(exp, sol.FindKthNumber(n, k));
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