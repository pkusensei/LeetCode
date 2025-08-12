using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(10, 13, 2)]
    [DataRow(1, 1, 1)]
    public void TestMethod1(int exp, int n, int k)
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