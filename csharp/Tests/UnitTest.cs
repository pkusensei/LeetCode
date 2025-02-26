using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, -3, 2, 3, -4 }, 5)]
    [DataRow(new[] { 2, -5, 1, -4, 3, -2 }, 8)]
    public void TestMethod1(int[] nums, int exp)
    {
        Assert.AreEqual(exp, sol.MaxAbsoluteSum(nums));
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