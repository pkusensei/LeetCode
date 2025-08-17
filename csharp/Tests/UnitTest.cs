using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(1.0, 10, 1, 10)]
    [DataRow(0.6, 6, 1, 10)]
    [DataRow(0.73278, 21, 17, 10)]
    public void TestMethod1(double exp, int n, int k, int mp)
    {
        Assert.IsTrue(double.Abs(exp - sol.New21Game(n, k, mp)) <= double.Pow(10, -5));
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