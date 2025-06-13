using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 10, 1, 2, 7, 1, 3 }, 2, 1)]
    [DataRow(new[] { 4, 2, 1, 2 }, 1, 0)]
    public void TestMethod1(int[] s, int p, int exp)
    {
        Assert.AreEqual(exp, sol.MinimizeMax(s, p));
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