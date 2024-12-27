using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 8, 1, 5, 2, 6 }, 11)]
    [DataRow(new int[] { 1, 2 }, 2)]
    public void TestMethod1(int[] nums, int exp)
    {
        Assert.AreEqual(exp, sol.MaxScoreSightseeingPair(nums));
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