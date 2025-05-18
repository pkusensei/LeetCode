using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(1, 1, 3)]
    [DataRow(1, 2, 6)]
    [DataRow(5, 5, 580986)]
    public void TestMethod1(int m, int n, long exp)
    {
        Assert.AreEqual(exp, sol.ColorTheGrid(m, n));
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