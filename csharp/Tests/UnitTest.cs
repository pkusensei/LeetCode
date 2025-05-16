using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(3, 5, 27)]
    [DataRow(1, 4, 2)]
    [DataRow(5, 6, 2468)]
    public void TestMethod1(int n, int k, long exp)
    {
        Assert.AreEqual(exp, sol.CountGoodIntegers(n, k));
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