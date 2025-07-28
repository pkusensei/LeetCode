using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 9, 8, 6, 5, 3 }, new[] { 3, 4, 6, 5 }, new[] { 9, 1, 2, 5, 8, 3 }, 5)]
    [DataRow(new[] { 6, 7, 6, 0, 4 }, new[] { 6, 7 }, new[] { 6, 0, 4 }, 5)]
    [DataRow(new[] { 9, 8, 9 }, new[] { 3, 9 }, new[] { 8, 9 }, 3)]
    [DataRow(new[] { 6, 3, 9, 2, 2, 5, 2, 1, 4, 4, 5, 7, 8, 9, 3, 1, 6, 9, 7, 0, 5, 6, 0 },
             new[] { 6, 3, 9, 0, 5, 6 }, new[] { 2, 2, 5, 2, 1, 4, 4, 5, 7, 8, 9, 3, 1, 6, 9, 7, 0 }, 23)]
    public void TestMethod1(int[] exp, int[] n1, int[] n2, int k)
    {
        Assert.IsTrue(exp.SequenceEqual(sol.MaxNumber(n1, n2, k)));
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