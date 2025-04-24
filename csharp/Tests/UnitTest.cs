using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 3, 1, 2, 2 }, 4)]
    [DataRow(new[] { 5, 5, 5, 5 }, 10)]
    public void TestMethod1(int[] nums, int exp)
    {
        Assert.AreEqual(exp, sol.CountCompleteSubarrays(nums));
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