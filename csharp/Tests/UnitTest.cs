using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(4, 2, 0, new[] { 1, 2, 3 }, new[] { 0, 1, 1 })]
    public void TestMethod1(int exp, int k, int w, int[] p, int[] c)
    {
        Assert.AreEqual(exp, sol.FindMaximizedCapital(k, w, p, c));
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