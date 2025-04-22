using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(2, 5, 10)]
    [DataRow(5, 3, 11)]
    public void TestMethod1(int n, int maxValue, int exp)
    {
        Assert.AreEqual(exp, sol.IdealArrays(n, maxValue));
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