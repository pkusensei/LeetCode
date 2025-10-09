using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(110, new[] { 1, 5, 2, 4 }, new[] { 5, 1, 4, 2 })]
    public void TestMethod1(long exp, int[] s, int[] m)
    {
        Assert.AreEqual(exp, sol.MinTime(s, m));
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