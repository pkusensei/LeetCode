using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(1, 3, "c")]
    [DataRow(1, 4, "")]
    [DataRow(3, 9, "cab")]
    public void TestMethod1(int n, int k, string exp)
    {
        Assert.AreEqual(exp, sol.GetHappyString(n, k));
        Assert.AreEqual(exp, sol.WithStack(n, k));
        Assert.AreEqual(exp, sol.WithMath(n, k));
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