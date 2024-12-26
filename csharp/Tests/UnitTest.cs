using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 1, 1, 1, 1, 1 }, 3, 5)]
    [DataRow(new int[] { 1 }, 1, 1)]
    public void TestMethod1(int[] nums, int target, int exp)
    {
        Assert.AreEqual(exp, sol.FindTargetSumWays(nums, target));
    }

    [TestMethod]
    [DataRow(new int[] { 1, 1, 1, 1, 1 }, 3, 5)]
    [DataRow(new int[] { 1 }, 1, 1)]

    public void TestMethod2(int[] nums, int target, int exp)
    {
        Assert.AreEqual(exp, sol.WithDp(nums, target));
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