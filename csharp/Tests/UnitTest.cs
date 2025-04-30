using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(3, 4, 6)]
    [DataRow(2, 7, 15)]
    [DataRow(1, 4, 4)]
    public void TestMethod1(int n, int x, int exp)
    {
        Assert.AreEqual(exp, sol.MinEnd(n, x));
    }

    [TestMethod]
    public void TestMethod2()
    { }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}