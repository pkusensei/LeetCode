using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 2, 3, 4, 5, 6, 7, 8 }, 5)]
    [DataRow(new[] { 1, 3, 7, 11, 12, 14, 18 }, 3)]
    public void TestMethod1(int[] nums, int exp)
    {
        Assert.AreEqual(exp, sol.LenLongestFibSubseq(nums));
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