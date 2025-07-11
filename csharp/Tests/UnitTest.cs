using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 1, 1, 2, 2, 2, 3, 3 }, new[] { 1, 1, 2, 2, 3, 3 })]
    [DataRow(new[] { 1, 2, 3, 3, 3 }, new[] { 1, 2, 3, 3 })]
    public void TestMethod1(int[] n, int[] exp)
    {
        Assert.AreEqual(exp.Length, sol.RemoveDuplicates(n));
        Assert.IsTrue(exp.SequenceEqual(n.Take(exp.Length)));
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