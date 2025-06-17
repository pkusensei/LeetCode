using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(3, 2, 1, 4)]
    [DataRow(4, 2, 2, 6)]
    [DataRow(5, 2, 0, 2)]
    public void TestMethod1(int n, int m, int k, int exp)
    {
        Assert.AreEqual(exp, sol.CountGoodArrays(n, m, k));
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