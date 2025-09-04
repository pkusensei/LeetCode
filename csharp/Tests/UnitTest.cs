using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(3, new[] { 1, 0, 5 })]
    [DataRow(2, new[] { 0, 3, 0 })]
    [DataRow(-1, new[] { 0, 2, 0 })]
    [DataRow(8, new[] { 0, 0, 11, 5 })]
    public void TestMethod1(int exp, int[] nums)
    {
        Assert.AreEqual(exp, sol.FindMinMoves(nums));
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