using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(10, 19, "[11,13]")]
    [DataRow(4, 6, "[-1,-1]")]
    public void TestMethod1(int a, int b, string exp)
    {
        Assert.AreEqual(exp, sol.ClosestPrimes(a, b).Print());
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