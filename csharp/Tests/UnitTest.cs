using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 2, 1, 1, 0 }, new[] { 5, 2, 6, 1 })]
    public void TestMethod1(int[] exp, int[] n)
    {
        Assert.IsTrue(exp.SequenceEqual(sol.CountSmaller(n)));
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