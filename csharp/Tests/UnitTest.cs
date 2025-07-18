using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(-1, new[] { 3, 1, 2 })]
    [DataRow(1, new[] { 7, 9, 5, 8, 1, 3 })]
    public void TestMethod1(int exp, int[] nums)
    {
        Assert.AreEqual(exp, sol.MinimumDifference(nums));
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