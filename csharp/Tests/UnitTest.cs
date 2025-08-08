using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(0.625, 5)]
    public void TestMethod1(double exp, int n)
    {
        Assert.AreEqual(exp, sol.SoupServings(n));
        Assert.AreEqual(exp, sol.BottomUp(n));
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