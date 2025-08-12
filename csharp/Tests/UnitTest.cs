using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(1, 10, 2)]
    [DataRow(2, 4, 1)]
    public void TestMethod1(int exp, int n, int x)
    {
        Assert.AreEqual(exp, sol.NumberOfWays(n, x));
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