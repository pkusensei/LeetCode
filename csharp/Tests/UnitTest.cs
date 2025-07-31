using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 0, 1, 1, 2, 1, 2 }, 5)]
    public void TestMethod1(int[] exp, int n)
    {
        Assert.IsTrue(exp.SequenceEqual(sol.CountBits(n)));
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