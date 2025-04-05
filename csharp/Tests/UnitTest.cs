using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 3 }, 6)]
    [DataRow(new[] { 5, 1, 6 }, 28)]
    [DataRow(new[] { 3, 4, 5, 6, 7, 8 }, 480)]
    public void TestMethod1(int[] nums, int exp)
    {
        Assert.AreEqual(exp, sol.SubsetXORSum(nums));
        Assert.AreEqual(exp, sol.WithBits(nums));
    }

    [TestMethod]
    public void TestMethod2()
    { }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}