using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(5, 5)]
    public void TestMethod1(int exp, int n)
    {
        Assert.AreEqual(exp, sol.FindIntegers(n));
        Assert.AreEqual(exp, sol.DigitDpWithFib(n));
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